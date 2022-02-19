# Dory

A lightweight key-value cache system developed for experimental purposes. It can also include distributed systems setup if I can.

## Main Idea

Dory's main idea is to use blocks that store key:value pairs in small size and pop into memory in different modes. For example, in basic mode, two packs with a maximum of 1000 key:values can be managed by two different threads. In Advanced mode, 16 different threads can share a total of 16000 key:value pairs.

## Message Structure

Dory uses her own private messages. The format of network packets coming to the server is in a certain standard.

- The commands that can be used are certain and cannot exceed 3 letters.
- The first [] contains the key name. Key lengths are fixed and cannot exceed 16 characters.
- If the ADD command is used, the third and last [] blocks are used.
  - The second [] contains object's type.
  - The last [] contains the object. The object length is shaped according to the type specified in the previous section.

**For example**

- ADD[ServerName][STRING][LOCALHOST]
- GET[ServerName]
- DEL[ServerName]
- ADD[Logs][BOOLEAN][0]
- ADD[DefaultPi][U32][3.1415]

## Common Features

todo();
