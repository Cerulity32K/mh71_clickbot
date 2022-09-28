# Geometry Dash Clickbot (for Mega Hack v7.1)
This program will create clicks as a wav file for a Mega Hack v7 macro.
## Tutorial
1. Create a Mega Hack v7 macro (on the Macro tab, press Record, record inputs, input a macro name and click Save)
2. Click on Advanced Options > Export JSON
3. Navigate to the folder the executable is contained in and save it as `macro.json`.
4. In the `assets` folder, put two wav files `down.wav` and `up.wav` into the folder. They need to have the same properties to properly work (same channel count, sample rate, etc.). `down.wav` will be the sound for clicking down, `up.wav` will be the sound for clicking up.
5. Run the program. If the macro is invalid, it will tell you. Otherwise, you should see an output.wav file after running the program.
6. Record the macro as a video (OBS Studio recommended).
7. Open any video editor.
8. Import the `output.wav` file and your recording of the level into seperate tracks.
9. Make sure the start of the audio file matches up with the first frame of the video.
10. Playback to test for sync.
11. Export, and you're finished!
