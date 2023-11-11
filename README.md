# Geometry Dash Clickbot (for Mega Hack v7.1)
This program will create clicks as a wav file for a Mega Hack v7 macro.

## Tutorial
1. Create a Mega Hack v7 macro (on the Macro tab, press Record, record inputs, input a macro name and click Save)
2. Click on Advanced Options > Export JSON
3. Navigate to the folder the executable is contained in and save it as whatever you want (recommended extension is `.mhr.json`).
4. In the `assets` folder, put two wav files `down.wav` and `up.wav` into the folder. They need to have the same properties to properly work (same channel count, sample rate, etc.). `down.wav` will be the sound for clicking down, `up.wav` will be the sound for clicking up.
5. Drag the macro file you saved onto the program (or run the program in the terminal with the first argument being the macro you saved). If the macro is invalid, it will tell you. Otherwise, you should see an output.wav file after running the program.
6. Record the macro as a video (OBS Studio recommended).
7. Open any video editor.
8. Import the `output.wav` file and your recording of the level into seperate tracks.
9. Make sure the start of the audio file matches up with the first frame of the video.
10. Playback to test for sync.
11. Export, and you're finished!

## Randomising Clicks
1. Create a file in the same folder as the executable called `downs.txt`.
2. Enter all the file names (separated by new lines) you want to randomly choose as down clicks.
3. Make sure all of those files exist in the `assets` folder.
5. The same can be done with `ups.txt` and release sounds.
4. The program can be run as normal.