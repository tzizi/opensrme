import struct
import sys
import pprint

filename = sys.argv[1]
read = list(open(filename, "rb").read())

pos = 0
palettes = []

def print_pos():
    print('0x%x'%(pos))

def read_boolean():
    global pos
    short = struct.unpack('>?', bytes(read[pos:pos+1]))[0]
    pos += 1
    return short

def read_byte():
    global pos
    short = struct.unpack('>b', bytes(read[pos:pos+1]))[0]
    pos += 1
    return short

def read_unsigned_byte():
    global pos
    short = struct.unpack('>B', bytes(read[pos:pos+1]))[0]
    pos += 1
    return short

def read_short():
    global pos
    short = struct.unpack('>h', bytes(read[pos:pos+2]))[0]
    pos += 2
    return short

def read_unsigned_short():
    global pos
    short = struct.unpack('>H', bytes(read[pos:pos+2]))[0]
    pos += 2
    return short

def read_int():
    global pos
    short = struct.unpack('>i', bytes(read[pos:pos+4]))[0]
    pos += 4
    return short

def read_utf():
    global pos
    length = read_short()
    string = bytes(read[pos:pos+length])
    pos += length
    return string

def read_palette():
    global palettes, pos

    palette_amt = read_short()
    palettes = []
    for i in range(palette_amt):
        size = read_int()
        # parse palettes
        pos += size
        #print(size)

read_palette()


font_amt = 0
font_images = []
font_sizes = []
font_offsets = []
iarr = []

def read_fonts():
    global font_images, font_sizes, font_offsets, iarr, pos
    font_amt = read_short()
    for i in range(font_amt):
        font_images.append(read_utf())

        somelen = read_short()

        font_sizes.append([])
        font_offsets.append([])

        for j in range(somelen):
            #somelen1 = struct.unpack('>I', bytes(read[pos:pos+4]))[0]
            pos += 4

            font_sizes[i].append([])
            font_offsets[i].append([])

            iarr.append(read_short())

            for k in range(256):
                font_offsets[i][j].append(read_short())

                font_sizes[i][j].append(read_short())
    """print(font_images)
    print(font_amt)
    print(font_sizes[0])
    print(font_sizes[1])
    print(font_offsets[0])
    print(font_offsets[1])
    print(iarr)
    print("0x%x"%pos)"""

    readShort3 = read_short()
    #print(readShort3)
    #print(font_offsets[0])

    #X = new image[readShort3]
    # more
    for i in range(readShort3):
        pos += 8


read_fonts()
print_pos()

languages = []

def read_strings():
    global languages
    unknowns = []
    languages_amt = read_short()
    for i in range(languages_amt):
        read_int()

        strings_amt = read_short()
        strings = []
        for j in range(strings_amt):
            strings.append(read_utf())

        languages.append(strings)
        unknowns.append(read_short())
    print(unknowns)

read_strings()

def lps(x):
    print(languages[0][x])
def lp(*args):
    for i in args:
        lps(i)


for i in range(len(sys.argv) - 2):
    lps(int(sys.argv[i+2]))


def read_images():
    images_len = read_short()
    arr1 = [] # sprite_info_offset
    arr2 = [] # boundingbox

    i = 1
    while i <= images_len:
        arr2.append(read_short())
        arr2.append(read_short())
        arr2.append(read_short())
        arr2.append(read_short())
        arr1.append(read_short())
        i += 1

    arr3 = [] # sprite_info
    print(len(arr1))

    for i in range(arr1[images_len - 1]):
        arr3.append(read_short())

    images_strings = []
    images_strings_len = read_short()
    for i in range(images_strings_len):
        images_strings.append(read_utf())

    return
    print(arr1)
    print("")
    print(arr2)
    print("")
    print(arr3)
    print("")
    print(images_strings)
    print(len(images_strings))

read_images()


def read_clips():
    clips_len = read_short()
    print('%i clips' % clips_len)
    clips = []
    for i in range(clips_len):
        clips_sub_len = read_short()
        clips_sub = []
        for j in range(clips_sub_len):
            clips_ssub_len = read_short()
            clips_ssub = []
            for k in range(clips_ssub_len):
                clips_ssub.append(read_short())
            clips_sub.append(clips_ssub)
        clips.append(clips_sub)
    #return
    for i in range(len(clips)):
        print(i)
        print(clips[i])

read_clips()


def read_sound():
    sounds_len = read_short()
    sound_files = []
    sound_mimes = []
    sound_priority = []
    sound_load = []
    for i in range(sounds_len):
        sound_files.append(read_utf())
        sound_mimes.append(read_utf())
        sound_priority.append(read_int())
        sound_load.append(read_boolean())
    return
    print(sound_files)
    print(sound_mimes)
    print(sound_priority)
    print(sound_load)

read_sound()


def read_items():
    items_len = read_short()
    items = []
    for i in range(items_len):
        item = []
        # 0 = type? 0 = weapon, 1 = food, 2 = addon? (gear, durability, colors)
        # 1 = price?
        # 2 = increment
        # 3 = max owned
        # 4 = item name id
        # 5 = item desc id
        # 6 = sprite id
        for i in range(6):
            item.append(read_int())
        item.append(read_short())
        items.append(item)
        continue
        print('%s %s' % (str(languages[0][item[4]]), str(item)))

read_items()


def read_quests():
    quests_len = read_short()
    quests = []
    for i in range(quests_len):
        quest = []
        # 0 = currently active?
        #     1 = ?
        #     2 = active
        #     4 = complete
        # 1 = person giving the quest (tid)
        # 2 = is mission start?
        # 3 = person sprite id
        # 4 = quest name (tid)
        # 5 = quest description (tid)
        # 6 = level id
        quest.append(0)
        quest.append(read_int())
        if read_boolean():
            quest.append(1)
        else:
            quest.append(0)
        quest.append(read_short())
        quest.append(read_int())
        quest.append(read_int())
        quest.append(read_int())
        quests.append(quest)

        continue
        pprint.pprint([
            languages[0][quest[1]],
            languages[0][quest[4]],
            languages[0][quest[5]],
            quest
        ])
read_quests()


def read_gangs():
    gangs_len = read_short()
    gangs = []
    for i in range(gangs_len):
        gang = []
        # 0 = gang name
        # 1 = sprite id
        # 2 = ?
        # 3 = default notoriety
        # 4 = ?
        gang.append(read_int())
        gang.append(read_short())
        gang.append(read_short())
        gang.append(read_byte())
        gang.append(read_int())
        gangs.append(gang)
        continue
        pprint.pprint([
            languages[0][gang[0]],
            gang
        ])
read_gangs()

print(pos)

def read_effects():
    effects_len = read_short()

    for effect_i in range(effects_len):
        # effect_i 9 = gore
        effect = {}

        effect["effect_type"] = read_int()
        effect_type = effect["effect_type"]
        effect["should_be_2"] = read_int()
        effect["unk1"] = read_int()
        effect["animation_time"] = read_unsigned_short()

        print(pos)

        if effect_type == 0: # clip
            effect["clip"] = read_int()
        if effect_type == 1: # spawner
            spawners_len = read_short()
            spawners = []
            for i in range(spawners_len):
                newarray = []
                for j in range(5):
                    newarray.append(0)
                # effect id
                newarray[3] = read_int()
                # increment amount (n / this = number of effects added)
                newarray[4] = read_unsigned_short()
                # 0-2: pos-orientation
                newarray[0] = read_int()
                newarray[1] = read_int()
                newarray[2] = read_int()
                spawners.append(newarray)
            effect["spawners"] = spawners
        if effect_type == 2:
            effect["linked_effect"] = read_int()
            array_len = read_short()

            array1 = []
            array2 = []

            for i in range(array_len):

                array2.append([
                    # 0: operation (0-3)
                    read_int(),
                    # 1: always 0?, adds to the current time elapsed for the operation
                    read_int(),
                    # 2: variable 0
                    read_int(),
                    # 3: variable 1
                    read_int()
                ])
                newarray = []
                newarray_len = read_short()
                for j in range(newarray_len * 2):
                    newarray.append(read_int())
                array1.append(newarray)
            effect["array1"] = array1
            effect["array2"] = array2
        if effect_type == 3:
            effect["color"] = '%x'%(read_int())
            effect["rect_size"] = read_unsigned_byte()
        if effect_type == 4:
            effect["color"] = '%x'%(read_int())
            effect["size"] = read_int()
        print(str(effect_i) + " " + str(effect_type))
        pprint.pprint(effect)
read_effects()
