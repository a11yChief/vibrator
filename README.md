# vibrator
## Ok, what the fuck's going on here?
Vibrator stands for vibecoded in rust accessibility tool in an open repository. I'm writing this on Christmas Eve 2025, it's been one hell of a year, and I wanted to have some fun torturing some AI models. (I am so going to hell)

Anyway, what better way to do that than gaslighting copilot with the borrow checker *and* making it feel bad for failing a bunch of blind people at the same time? Plus, I am extremely against vibe-coding as a rule. Junior developers with unbridled passion for architecting and writing great software would long-term annihilate any AI model over time, so I'll consider this a real world proof.

## Repository rules
0. Do not use any of this code in a production environment. I know I chose the GPL2, just don't even think about using this as a basis for a project. Fork, contribute, build and run at your own risk, but good lord save us all plese be careful.
1. All code must be written by AI.
2. There must be genuine attempts to keep this project building, no matter what it takes.
3. This is a Linux Screen Reader, and it must work on modern linux stacks. X11 should not be supported. Wayland Only, and you'll see why when you explain this to your AI of choice.
4. Attempts must be made to have this be as useful as possible. Feature Creap in the style of an overexcited, underexperienced product manager with the slimiest of GPT models egging them on with sycophancy is highly encouraged behaviour, but the project must always run and build.
5. Tests must be entirely written by AI.
6. Do not poison the project. As you might have guessed by now, this is all in good fun. It's supposed to be cathartic for all the times AI drove us crazy with mistakes the worst of us wouldn't have made high at the bottom of a ditch, with a bottle in one hand and a lighter in the other. If you deliberately try and ruin the fun for anyone here, you will be banned.
7. Keep it civil. In PRs, issues, anything really; I mean there's enough war going on at the moment without us adding keyboard warriors to the mix. If you disagree with another user's opinion, respect that others hav different opinions to you. We aren't all made to get along, but we do have the power to be civil and find common ground where we can.

## How to get involved
If you want to support this madness in any number of ways, I'd be extremely greatful. Feel free to star the project, or become a sponsor. If you want to join in the fun, you will need the following tools:
* A computer of some description running a Linux of some variety. My personal setup is Arch BTW, using the LTS Kernel. My Desktop Environment is Gnome. For non-visually impaired contributers, it's almost impossible to get a screen reader working on Hyprland or similar window managers, not even Orca can do it.
* Rustup with the default stable toolchain.
* Some form of AI code thing that will do all the work for you while you drink coffee and play Counterstrike/Halo/Just Cause/The Vail Shadow of the Crown/The Stanley Parable.
* A soft pad or pillow for when your head wishes to violently approach your desk in frustration.

## Note for non-visually impaired contributers
Hopefully this also serves as a nod to how hard accessibility is on Linux. I've seen the official Orca screen reader work perfectly on my arch setup with vanilla settings, but break on openSUSE Tumbleweed with their settings, with the same exact desktop environment. If you want to learn more about screen readers in general, how we use them, please create an issue with the Question label.

Hopefully by building the world's worst screen reader, we can educate a few people along the way.

With all my best wishes, hopes for a brighter future and many alcohols to come,

Richard Hyman.