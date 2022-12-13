import os
import telebot
import requests
import json
import csv
from dotenv import load_dotenv,find_dotenv

load_dotenv(find_dotenv())

# TODO: 1.1 Get your environment variables 
yourkey = os.getenv("KEY")
bot_id = os.getenv("ID")
token = os.getenv("TOKEN")
bot = telebot.TeleBot(token)






@bot.message_handler(commands=['start', 'hello'])
def greet(message):
    global botRunning
    botRunning = True
    bot.reply_to(
        message, 'Hello there! I am a bot that will show movie information for you and export it in a CSV file.\n\n')


    
@bot.message_handler(commands=['stop', 'bye'])
def goodbye(message):
    global botRunning
    botRunning = False
    bot.reply_to(message, 'Bye!\nHave a good time')



@bot.message_handler(func=lambda message: botRunning, commands=['help'])
def helpProvider(message):
    bot.reply_to(message, '1.0 You can use \"/movie MOVIE_NAME\" command to get the details of a particular movie. For eg: \"/movie The Shawshank Redemption\"\n\n2.0. You can use \"/export\" command to export all the movie data in CSV format.\n\n3.0. You can use \"/stop\" or the command \"/bye\" to stop the bot.')




@bot.message_handler(func=lambda message: botRunning, commands=['movie'])
def getMovie(message):
 bot.reply_to(message , 'Getting movie info...')



    # TODO: 1.2 Get movie information from the API
 movie_name = (message.text.split(' ', 1)[1])
 response = requests.get('http://www.omdbapi.com/?i=tt3896198&apikey='+str(yourkey)+'&t='+movie_name)
 data = response.json()
 title = data["Title"]
 year = data["Year"]
 rating = data["imdbRating"]
 poster= data["Poster"]
 bot.send_photo(message.chat.id, poster)
 bot.reply_to(message, f"Title: {title}\nYear: {year}\nRating: {rating}")



    # TODO: 2.1 Create a CSV file and dump the movie information in it
 with open('movie.csv', 'w') as csvfile:
    fieldnames = ['Title', 'Year', 'imdbRating']
    writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
    writer.writeheader()
    writer.writerow({'Title': title, 'Year': year, 'imdbRating': rating})



@bot.message_handler(func=lambda message: botRunning, commands=['export'])
def getList(message):
    bot.reply_to(message, 'Generating file...')
    #TODO: 2.2 Send downlodable CSV file to telegram chat
    with open('movie.csv', 'rb') as csvfile:
        bot.send_document(chat_id=message.chat.id,document=open(r'movie.csv', 'rb'))
        bot.reply_to(message, 'CSV file generated!You can download it now.')



@bot.message_handler(func=lambda message: botRunning)
def default(message):
    bot.reply_to(bot, 'I did not understand '+'\N{confused face}')
bot.infinity_polling()

