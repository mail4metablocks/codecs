package main

import (
	"fmt"
	"io"
	"log"
	"os"

	"github.com/sahilm/ffmpeg-go"
)

func main() {
	inputFile, err := os.Open("input.mp4")
	if err != nil {
		log.Fatal(err)
	}
	defer inputFile.Close()

	outputFile, err := os.Create("output.mp4")
	if err != nil {
		log.Fatal(err)
	}
	defer outputFile.Close()

	decoder, err := ffmpeg.NewDecoder(inputFile)
	if err != nil {
		log.Fatal(err)
	}
	defer decoder.Close()

	encoder, err := ffmpeg.NewEncoder(outputFile)
	if err != nil {
		log.Fatal(err)
	}
	defer encoder.Close()

	for {
		packet, err := decoder.Decode()
		if err == io.EOF {
			break
		} else if err != nil {
			log.Fatal(err)
		}

		err = encoder.Encode(packet)
		if err != nil {
			log.Fatal(err)
		}
	}

	fmt.Println("Finished transcoding")
}
