import requests
# import aiohttp
# import asyncio

data = {"map": {"Sophia Kera":["Alis Akelyan","Isaac Johnson","Mike Agzamov"],"Alis Akelyan":["Isaac Johnson","Mike Agzamov","Sophia Kera"],"Isaac Johnson":["Mike Agzamov","Sophia Kera","Alis Akelyan"],"Mike Agzamov":["Sophia Kera","Alis Akelyan","Isaac Johnson"],"Olivia Shin":["Nazeli Bari","Scotland Hughes","Naiya Rothermund"],"Nazeli Bari":["Scotland Hughes","Naiya Rothermund","Olivia Shin"],"Scotland Hughes":["Naiya Rothermund","Olivia Shin","Nazeli Bari"],"Naiya Rothermund":["Olivia Shin","Nazeli Bari","Scotland Hughes"],"Andrew Suda":["Paul Lee","Aidan Martinez","Deven Shah"],"Paul Lee":["Aidan Martinez","Deven Shah","Andrew Suda"],"Aidan Martinez":["Deven Shah","Andrew Suda","Paul Lee"],"Deven Shah":["Andrew Suda","Paul Lee","Aidan Martinez"],"Lauren Lee":["Justin Kim","Isagani Cabrera","Maya Taschyan"],"Justin Kim":["Isagani Cabrera","Maya Taschyan","Lauren Lee"],"Isagani Cabrera":["Maya Taschyan","Lauren Lee","Justin Kim"],"Maya Taschyan":["Lauren Lee","Justin Kim","Isagani Cabrera"],"Natalie Barcena":["Emma Hovsepian","Dana Kim","Travis Phillips","Isaac Johnson"],"Emma Hovsepian":["Dana Kim","Travis Phillips","Natalie Barcena"],"Dana Kim":["Travis Phillips","Natalie Barcena","Emma Hovsepian"],"Travis Phillips":["Natalie Barcena","Emma Hovsepian","Dana Kim"],"Anaya":["Payton","Sofia","Jesslyn"],"Payton":["Sofia","Jesslyn","Anaya"],"Sofia":["Jesslyn","Anaya","Payton"],"Jesslyn":["Anaya","Payton","Sofia"]}}


group_size = 4
num_groups = 6

url = f"http://127.0.0.1:3000/compute/{group_size}/{num_groups}"

response = requests.post(url, json=data)

# async def fetch(session, url, data):
#     async with session.post(url, json=data) as response:
#         return await response.text()

# async def main():
#     async with aiohttp.ClientSession() as session:
#         tasks = [fetch(session, url, data) for _ in range(100)]
#         responses = await asyncio.gather(*tasks)
#         for response in responses:
#             print(response)

# asyncio.run(main())

print(response.text)