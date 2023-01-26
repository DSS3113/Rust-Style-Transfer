# Rust-Style-Transfer

Credits: <a href="https://github.com/DSS3113">Danish Singh Sethi</a>, <a href="https://github.com/ethvedbitdesjan">Vedaant Jain</a>, <a href="https://github.com/henopied">Jake Mayer</a>

## Project Introduction: 

- Artistic Style Tranfer Using CNN

- The project aims to develop a CNN that takes two images (content image and style image) as input and produces an output image that incorporates the content image's content using the artistic style present in the style image. For example,

![This is an image](https://2.bp.blogspot.com/-kV4SKTFlWQk/WA6n82yFFJI/AAAAAAAABWY/9GcePSQZ7qcY95b7zVnCBR4ABWR7K2o4gCLcB/s1600/image04.png)

- The trained model uses a REST API in Rust to host the model which can be accessed through a server.

### Technical Overview:
This project:
- Uses CNNs for artistic style transfer.
- Uses the Rust bindings of PyTorch to develop a CNN.
- Uses the GPU to train the model in Rust.
- Uses a REST API to interface between the user and the server which hosts the model.

### Working Demo:


https://user-images.githubusercontent.com/50327599/214757930-e589995f-d3be-4bfe-8dd2-11a5de4b00e8.mp4



##### References
- https://www.cv-foundation.org/openaccess/content_cvpr_2016/papers/Gatys_Image_Style_Transfer_CVPR_2016_paper.pdf
