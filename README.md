# Dory

A lightweight key-value cache system developed for experimental purposes. It can also include distributed systems setup if I can.

## Main Idea

Dory's main idea is to use blocks that store key:value pairs in small size and pop into memory in different modes. For example, in basic mode, two packs with a maximum of 1000 key:values can be managed by two different threads. In Advanced mode, 16 different threads can share a total of 16000 key:value pairs.

## Common Features

todo();
