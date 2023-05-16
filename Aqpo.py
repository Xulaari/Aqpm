aqpoversion = 2316

import requests, sys, webbrowser
from tkinter import messagebox

version = requests.get("https://setup.rbxcdn.com/version").text

robloxurl = f"https://setup.rbxcdn.com/{version}-Roblox.exe"

apps = requests.get("https://pastebin.com/raw/ipRUyBCL").json()
apps["roblox"] = robloxurl

if (
    aqpoversion != apps["aqpoversion"]
    and messagebox.askyesno(
        title="Aqpo",
        message=f"This version: {aqpoversion} is out of date. Want to upgrade?",
    )
    == True
):
    webbrowser.open("https://github.com/Xulaari/Aqpo")
    sys.exit()
else:
    pass

print(
    "Aqpo Package Manager [Version 230515]\n(c) 2023 ACHROMATIC LTD. All Rights Reserved.\n"
)

import os, subprocess

usercontinuation = ""

uinput = input(
    f"Aqpo\\{os.getlogin()}> Enter the application you want to download or h for help: "
).lower()

if uinput in apps:
    usercontinuation = input(
        f"Aqpo\\{os.getlogin()}> {uinput.capitalize()} selected, Do you want to continue? (y/n): "
    )
else:
    pass

if uinput == "h":
    print(", ".join(apps)[13:])
if uinput in apps and usercontinuation == "y".lower():
    print(f"Aqpo\\{os.getlogin()}> Installing: {uinput.capitalize()}")
    url = apps[uinput]
    with open(f"{uinput.capitalize()}.exe", "wb") as nv:
        nv.write(requests.get(url).content)
    wantstopen = input(
        f"Aqpo\\{os.getlogin()}> Do you want to open {uinput.capitalize()}? (y/n): "
    )
    if wantstopen == "y":
        subprocess.Popen(f"{uinput.capitalize()}.exe")
    elif wantstopen == "n":
        print("Aqpo\\{os.getlogin()}> Thank you for using the Aqpo package manager!")
    else:
        print("Invalid input.")
elif usercontinuation == "n".lower():
    print("Aqpo\\{os.getlogin()}> Installation stopped.")
