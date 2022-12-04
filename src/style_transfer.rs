// This is inspired by the Neural Style tutorial from PyTorch.org
//   https://pytorch.org/tutorials/advanced/neural_style_tutorial.html
static style_layers_indices: [usize; 5] = [0, 2, 5, 7, 10];
static content_layers_indices: [usize; 1] = [7];
use tch::Tensor;
use tch::vision::vgg;
use tch::Device;
use tch::vision::imagenet;
use anyhow::bail;
use std::env;
use tch::nn::OptimizerConfig;
use anyhow::Result;
use tch::nn;
fn gram_matrix(matrix : &Tensor) -> Tensor {
    let (batch_size, d, h, w) = matrix.size4().unwrap();
    let intrmedit = matrix.view((batch_size * d, h * w));
    let output = intrmedit.matmul(&intrmedit.tr());
    return output / (batch_size * d * h * w);
}

fn style_loss_fn(matrix0 : &Tensor, matrix1 : &Tensor) -> Tensor{
    return gram_matrix(matrix0).mse_loss(&gram_matrix(matrix1), tch::Reduction::Mean);
}

pub async fn style_transfer(uuid : String, style_file_name : String, content_file_name : String, weights : String) -> Result<()> {
    let device = Device::cuda_if_available();
    let args: Vec<_> = env::args().collect();
    //println!("{}", env::var("RUST_BACKTRACE").unwrap());
    //println!("{}", env::var("PYTORCH_NO_CUDA_MEMORY_CACHING").unwrap());
    if device == Device::Cpu {
        println!("Running on CPU");
    } else {
        println!("Running on GPU");
    }
    let mut varstore_model = tch::nn::VarStore::new(device);
    let net = vgg::vgg16(&varstore_model.root(), imagenet::CLASS_COUNT);
    varstore_model.load(&weights).unwrap();
    varstore_model.freeze();

    let style_file_name = imagenet::load_image(&style_file_name)
        .unwrap()
        .unsqueeze(0)
        .to_device(device);
    let content_file_name = imagenet::load_image(&content_file_name)
        .unwrap()
        .unsqueeze(0)
        .to_device(device);
    let final_layer = style_layers_indices.iter().max().unwrap() + 1;
    let style_layers = net.forward_all_t(&style_file_name, false, Some(final_layer));
    let content_layers = net.forward_all_t(&content_file_name, false, Some(final_layer));

    let varstore = nn::VarStore::new(device);
    let input = varstore.root().var_copy("image_trial", &content_file_name);
    let mut optim = nn::Adam::default().build(&varstore, 1.5e-1)?;

    for i in 0..1000 {
        let input_layers = net.forward_all_t(&input, false, Some(final_layer));
        let style_loss: Tensor =
            style_layers_indices.iter().map(|&idx| style_loss_fn(&input_layers[idx], &style_layers[idx])).sum();
        let content_loss: Tensor = content_layers_indices
            .iter()
            .map(|&idx| input_layers[idx].mse_loss(&content_layers[idx], tch::Reduction::Mean))
            .sum();
        drop(input_layers);
        let total_loss = style_loss * 1e8 + content_loss;
        optim.backward_step(&total_loss);
        if i == 999 {
            println!("Loss: {}", f64::from(total_loss));
            imagenet::save_image(&input, &format!("./outputs/{}_output.jpg", uuid))?;
        } else {
            drop(total_loss);
        }
    }
    
    Ok(())
}

pub fn main() -> Result<()> {
    Ok(())
}
