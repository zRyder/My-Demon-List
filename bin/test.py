
import gd
import pymysql
mydb = pymysql.connect(
  host="localhost",
  user="root",
  password="PasswordHere",
  database="sys"
)

client = gd.Client()

async def print_hi():
    mycursor = mydb.cursor()
    filter = gd.Filters(difficulty=-2, strategy= 4)
    count = 0
    try:
        for level in await client.search_levels(filters=filter, pages=range(500)):
            diff = level.difficulty,
            sql = "INSERT INTO GDLEVELTEST (Id, Name, Creator, Demondiff) VALUES ('{}', '{}', '{}', '{}')".format(level.id, level.name, level.creator, level.difficulty)
            mycursor.execute(sql)
            count +=1

        print("Done ", count)
        mydb.commit()
        mycursor.close()

    except pymysql.Error as error:
        print("Failed to insert record into Laptop table {}".format(error))
        mycursor.close()


client.run(print_hi())
