# CS128H-Project-The Indomitable Dominators 

Group Member Names: Jake Mayer, Vedaant Jain, Danish Singh Sethi

Group Member Net IDs: vvjain3, dssethi2, jmmayer3

## Project Introduction: 

- Artistic Style Tranfer Using CNN

- The project aims to develop a CNN that takes two images (content image and style image) as input and produces an output image that incorporates the content image's content using the artistic style present in the style image. For example,

![This is an image](https://2.bp.blogspot.com/-kV4SKTFlWQk/WA6n82yFFJI/AAAAAAAABWY/9GcePSQZ7qcY95b7zVnCBR4ABWR7K2o4gCLcB/s1600/image04.png)

- After training the model, we will develop a REST API in Rust to host the model which can be accessed through a server.

### Technical Overview:
- Understand how to use CNNs for artistic style transfer.
- Use the Rust bindings of a framework like PyTorch or Tensorflow to develop a CNN.
- Use GPU to train the model in Rust.
- Develop a REST API to host the model and get output image by passing two images through a POST request.

#### Planned Roadmap
- Checkpoint 1: Learn to use a Deep Learning framework in Rust
- Checkpoint 2: Train the CNN for artistic style transfer
- Final Submission: Develop the API and host the model.

##### Possible Challenges
- Collecting relevant data and traning the model on a local GPU

##### References
- https://www.cv-foundation.org/openaccess/content_cvpr_2016/papers/Gatys_Image_Style_Transfer_CVPR_2016_paper.pdf
######
- We chose to work on this project as the three of us have always been curious about Neural Networks and their applications to images. Therefore, this project would give us ample opportunity to train a convolutional neural network and gain experience hosting and running a model. We would also gain some experience related to APIs and web programming in Rust through this project.
