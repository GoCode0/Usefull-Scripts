from gtts import gTTS
import json

# with open('/home/gocode/scripts/python/news.json') as f:
#     data = json.load(f)
txt = "This is a test of the library gTTS"
# for i in range(10):
#     r = str(data[i])
#     txt += r.split(":")[1].strip().strip("'\"}")
#     txt += "\n"
# print(txt)

language = "en"

speech = gTTS(text=txt, lang=language, slow=False)
speech.save("voice.mp3")
