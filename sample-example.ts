import { FfmpegStreamer } from "ffmpeg-streamer";

const inputFile = "input.mp4";
const outputFile = "output.mp4";

const streamer = new FfmpegStreamer({
  input: inputFile,
  output: outputFile,
});

streamer.on("start", () => {
  console.log("Started transcoding");
});

streamer.on("progress", (progress) => {
  console.log(`Transcoding progress: ${progress}%`);
});

streamer.on("end", () => {
  console.log("Finished transcoding");
});

streamer.on("error", (error) => {
  console.error(`Error transcoding: ${error}`);
});

streamer.start();
