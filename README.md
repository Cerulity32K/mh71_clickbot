#Inquiries
Discord: Cerulity#1049

# Geometry Dash Clickbot (for Mega Hack v7.1)
This program will create clicks as a wav file for a Mega Hack v7.1 macro.
## Tutorial
Make sure you have downloaded the executable. You can find it in the Releases section.
1. Set up your workspace:
* It is recommended to create a folder and place the executable inside.
* In the location of the executable, create a folder named `assets`. Sound bits are saved here.
2. Create a Mega Hack Macro:
* Record your macro how you would normally record a macro (if you've never recorded a macro, simply click the Record toggle and play the level to the end).
* Click Save to save the macro.
3. Export the macro:
* Click the Advanced Options combo box, then click the Export JSON button.
* In the window that pops up, navigate to the location of your executable and save the file as `macro.json` (make sure the file extension is not .mhr.json).
4. Click sounds:
* With your choice of recording software (as long as it can export to .wav, Audacity recommended), record the sound that plays when your mouse is clicked down. Export this to the location of the executable inside the `assets` folder created in step 1, and call it `down.wav`.
* Repeat, but with the sound that will play on mouse up, and call it `up.wav`.
5. Run the program:
* Your directory structure should now look like this (directory contents in {braces}):
```
<folder name> {
    mh71_clickbot.exe
    assets {
        down.wav
        up.wav
    }
    macro.json
}
```
* Double click the executable to run the program. The clickbot should find `macro.json`. If no error occurs, a new file should appear named `out.wav`. See Troubleshooting if the file does not appear.
6. Record the macro:
* Open your recording software of choice (OBS Studio recommended).
* Record a video of the macro in it's entirety.
* Save the video.
7. Edit the video:
* Open any video editor (Shotcut recommended, as it's simple, but use one you know how to use).
* Import your recording of the macro.
* Import the `out.wav` folder in the location of your executable.
* Line up the first frame of the macro with the start of `out.wav`.
* Export and save the edited video.
