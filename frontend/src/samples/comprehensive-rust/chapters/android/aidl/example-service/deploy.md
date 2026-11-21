# 배포

서비스를 빌드하고, 가상 디바이스에 넣고, 시작 할 수 있습니다:

``` shell
m birthday_server
adb push "$ANDROID_PRODUCT_OUT/system/bin/birthday_server" /data/local/tmp
adb root
adb shell /data/local/tmp/birthday_server
```

다른 터미널을 띄워서 서비스가 잘 수행되고 있는지 확인합니다:

``` shell
adb shell service check birthdayservice
```

``` text
Service birthdayservice: found
```

`service call`명렁어로 서비스를 호출할 수도 있습니다:

``` shell
adb shell service call birthdayservice 1 s16 Bob i32 24
```

``` text
Result: Parcel(
  0x00000000: 00000000 00000036 00610048 00700070 '....6...H.a.p.p.'
  0x00000010: 00200079 00690042 00740072 00640068 'y. .B.i.r.t.h.d.'
  0x00000020: 00790061 00420020 0062006f 0020002c 'a.y. .B.o.b.,. .'
  0x00000030: 006f0063 0067006e 00610072 00750074 'c.o.n.g.r.a.t.u.'
  0x00000040: 0061006c 00690074 006e006f 00200073 'l.a.t.i.o.n.s. .'
  0x00000050: 00690077 00680074 00740020 00650068 'w.i.t.h. .t.h.e.'
  0x00000060: 00320020 00200034 00650079 00720061 ' .2.4. .y.e.a.r.'
  0x00000070: 00210073 00000000                   's.!.....        ')
```
