import requests
import urllib.request
import time
from bs4 import BeautifulSoup
import subprocess as sp
import time
from termcolor import colored, cprint
from colorama import Fore, Back, Style 
old = 0
minute = time.time()+60
hour = time.time()+3600
day = time.time()+86400
week = time.time()+604800
response = requests.get("https://teamtrees.org")
soup = BeautifulSoup(response.text, "html.parser")
trees = soup.find("div", {"id": "totalTrees"})
count = trees.attrs['data-count']
minute_amount = count
hour_amount = count
day_amount = count
week_amount = count
while True:
  response = requests.get("https://teamtrees.org")
  soup = BeautifulSoup(response.text, "html.parser")
  trees = soup.find("div", {"id": "totalTrees"})
  count = trees.attrs['data-count']
  if old != count:
    if time.time()<minute:
      minute_amount = int(count) - int(minute_amount)
    else:
      minute = time.time()+60
      minute_amount = count
    if time.time()<hour:
      hour_amount = int(count) - int(hour_amount)
    else:
      hour = time.time()+3600
      hour_amount = count
    if time.time()<day:
      day_amount = int(count) - int(day_amount)
    else:
      day = time.time()+86400
      day_amount = count
    if time.time()<week:
      week_amount = int(count) - int(week_amount)
    else:
      week = time.time()+604800
      week_amount = count

    



    sp.call('clear',shell=True)
    print(f"""
    {Fore.RED}             #TeamTrees

    {Fore.GREEN}             ,@@@@@@@,
    {Fore.GREEN}     ,,,.   ,@@@@@@/@@,  .oo8888o.
    {Fore.GREEN}  ,&%%&%&&%,@@@@@/@@@@@@,8888\88/8o
    {Fore.GREEN} ,%&\%&&%&&%,@@@\@@@/@@@88\88888/88'
    {Fore.GREEN} %&&%&%&/%&&%@@\@@/ /@@@88888\88888'
    {Fore.GREEN} %&&%/ %&%%&&@@\ V /@@' `88\8 `/88'
    {Fore.GREEN} `&%\ ` /%&' {Fore.YELLOW}   |.|   {Fore.GREEN}     \ '|8'
    {Fore.YELLOW}     |o|        | |         | |
    {Fore.YELLOW}     |.|        | |         | |
    {Fore.GREEN}   \\{Fore.YELLOW}/ ._\{Fore.GREEN}//_/__{Fore.YELLOW}/  ,\{Fore.GREEN}_//__\\{Fore.YELLOW} /.  \{Fore.GREEN}_//__/_
    """)
    print("          "+colored(count[:2]+","+ count[2:5]+ ","+count[5:], "white", attrs=['bold', 'blink'])+ colored(" trees donated total ", "green")+ str(minute_amount) + "in the last minute" + "\n")

    print("           "+colored("Donate at", "red")+ colored(" teamtrees.org", "green", attrs=['bold']))
    old = count
    time.sleep(1)
  else:
    continue
