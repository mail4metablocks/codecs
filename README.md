# codecs
Streaming and Compression Algorithms


Codecs are algorithms that are used to compress and decompress digital audio and video data. They allow for the efficient storage and transmission of media files, as well as for their playback on various devices.

There are many different codecs available, each with its own specific set of features and characteristics. Some codecs are designed for high-quality audio and video, while others are optimized for low file sizes and fast transmission.

In the context of streaming, codecs are used to encode the audio and video data before it is transmitted over the internet. The encoded data is then decoded by the media player on the viewer's device, allowing the media to be played back.

There are many different codecs that are commonly used for streaming, including H.264, HEVC, VP9, and AV1. Each codec has its own advantages and disadvantages, and the choice of codec can have a significant impact on the quality and performance of the streaming experience.

In addition to audio and video codecs, there are also codecs for other types of data, such as image and text codecs. These codecs are used to compress and decompress data for a variety of applications, including web and mobile development, data storage, and data transmission.

## Implementation

The code samples opens an input file and an output file, and uses the FFmpeg library to decode the frames from the input file and encode them into the output file. The input file is assumed to be an MP4 file, but you can use any format that FFmpeg supports. The output file will also be an MP4 file, but you can change the format by using a different output format in the Format::output function.

The code uses the Type::Video constant to select the best video stream from the input file, and creates a new video stream in the output file using the same codec as the input stream. It then sets up the decoding and encoding contexts by setting the width and height of the frames, and uses a loop to read packets from the input file, decode them into frames, encode the frames back into packets, and write the packets to the output file.
