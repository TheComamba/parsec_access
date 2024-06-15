
from bs4 import BeautifulSoup
import os
import requests

URL = "https://people.sissa.it/~sbressan/CAF09_V1.2S_M36_LT/"

def assure_dev_data_folder():
    if not os.path.exists('dev_data'):
        os.makedirs('dev_data')

def collect_archive_names():
    web_page = requests.get(URL).text
    soup = BeautifulSoup(web_page, 'html.parser')
    archive_names = []
    for link in soup.find_all('a'):
        href = link.get('href')
        if href and href.endswith('.tar.gz'):
            archive_names.append(href)
    return archive_names

def main():
    assure_dev_data_folder()
    archive_names = collect_archive_names()
    print(archive_names)

main()
