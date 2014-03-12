from rflib import *
from itertoolsmodule import chain
from bitarray import bitarray
import time
import toml
import socket

names = toml.loads(open("outlets.toml").read())['db']
s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
d = RfCat(0)

s.bind(('localhost', 9997))

d.setFreq(433.9e6)

d.setMdmSyncMode(SYNCM_NONE)
d.setMdmModulation(MOD_ASK_OOK)

d.setMdmDRate(int(1.0/0.000187))

d.setMaxPower()

print d.reprRadioConfig()

out = []


while True:
	for x in names[s.recv(64)]:
		if x == 0:
			out += [1,0,0,0]
		if x == 1:
			out += [1,1,1,0]
	out += [0]*30
	d.RFxmit('\xff\xff\xff\xff')
	d.RFxmit(bitarray(out).tobytes(), repeat=5)

