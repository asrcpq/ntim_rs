from pypinyin import lazy_pinyin
from pypinyin.contrib.tone_convert import to_normal
def word2zrm(w):
	l = lazy_pinyin(w)
	for ll in l:
		if not ll.isalpha() or not ll.isascii():
			return None
	result = []
	for ll in l:
		z = pinyin2zrm(ll)
		if len(z) != 2:
			raise Exception(f"{ll} {w}")
		result.append(z)
	return "".join(result)

yydict = {
	"a": "aa",
	"e": "ee",
	"o": "oo",
	"u": "uu",
	"ai": 'ai',
	"ei": 'ei',
	"er": 'er',
	"ao": 'ai',
	"ou": 'ou',
	"an": 'an',
	"en": 'en',
	"ang": 'ag',
	"eng": 'eg',
	"ong": 'og',
}

def pinyin2zrm(p):
	if p.startswith("zh"):
		return "v" + yz(p.removeprefix("zh"))
	if p.startswith("ch"):
		return "i" + yz(p.removeprefix("ch"))
	if p.startswith("sh"):
		return "u" + yz(p.removeprefix("sh"))
	if p[0] in "qwrtypsdfghjklzxcbnm":
		if len(p) == 1:
			return p[0] + p[0]
		return p[0] + yz(p[1:])
	return yydict[p]

ydict = {
	"ai": 'l',
	"ei": 'z',
	"ui": 'v',
	"ao": 'k',
	"ou": 'b',
	"iu": 'q',
	"an": 'j',
	"en": 'f',
	"in": 'n',
	"un": 'p',
	"vn": 'p',
	"ang": 'h',
	"eng": 'g',
	"ing": 'y',
	"ong": 's',
	"uan": 'r',
	"uai": 'y',
	"ia": 'w',
	"ian": 'm',
	"iao": 'c',
	"ie": 'x',
	"uang": 'd',
	"iang": 'd',
	"iong": 's',
	"uo": 'o',
	"ua": 'w',
	"ue": 't',
	"ve": 't',
}

def yz(y):
	if len(y) == 1 and y in "aeiouvm":
		return y
	if y in ydict:
		return ydict[y]
	raise Exception(y)
