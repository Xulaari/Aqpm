import requests, subprocess
from tqdm import tqdm

usercontinuation = ""

print(
    "Aqpo Package Manager [Version 230515]\n©️ 2023 ACHROMATIC LTD. All Rights Reserved.\n"
)

uinput = input("> Enter the application you want to download or h for help: ").lower()

version = requests.get("https://setup.rbxcdn.com/version").text

robloxurl = f"https://setup.rbxcdn.com/{version}-Roblox.exe"

apps = requests.get("https://pastebin.com/raw/ipRUyBCL").json()
apps["roblox"] = robloxurl

if uinput in apps:
    usercontinuation = input(
        f"> {uinput.capitalize()} selected, Do you want to continue? (y/n): "
    )
else:
    pass

if uinput == "h":
    print(", ".join(apps))
if uinput in apps and usercontinuation == "y".lower():
    print(f"> Installing: {uinput.capitalize()}")
    url = apps[uinput]
    with open(f"{uinput.capitalize()}.exe", "wb") as nv:
        nv.write(requests.get(url).content)
    wantstopen = input(f"> Do you want to open {uinput.capitalize()}? (y/n): ")
    if wantstopen == "y":
        subprocess.Popen(f"{uinput.capitalize()}.exe")
    elif wantstopen == "n":
        print("> Thank you for using the Aqpo package manager!")
    else:
        print("Invalid input.")
elif usercontinuation == "n".lower():
    print("> Installation stopped.")
