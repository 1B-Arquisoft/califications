# README TO WORK ON DOCKER

## Build the image

```bash
docker build -t califications .
```

## Run the container

```bash
docker run -p 9005:8000 califications
```

REMEMBER: Rocket sets directions to 0.0.0.0:8000 as default
