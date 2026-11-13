import copy, zipfile, xml.etree.ElementTree as E
from pathlib import Path
out=Path(__file__).resolve().parent
ns={'hp':'http://www.hancom.co.kr/hwpml/2011/paragraph'}
with zipfile.ZipFile(out/'source.hwpx') as z:
 entries=[(copy.copy(i),z.read(i.filename)) for i in z.infolist()]
r=E.fromstring(dict((i.filename,b) for i,b in entries)['Contents/section0.xml'])
sub=r.find('.//hp:tc/hp:subList',ns);sub.set('vertAlign','TOP')
p1=copy.deepcopy(sub.find('hp:p',ns));p1.set('id','9032')
pic=p1.find('.//hp:pic',ns)
for k,v in [('id','9032'),('instid','9032'),('zOrder','4')]:pic.set(k,v)
for tag in ('sz','curSz'):
 for k in ('width','height'):pic.find('hp:'+tag,ns).set(k,'1200')
pic.find('hp:pos',ns).set('horzOffset','7800')
sub.insert(1,p1)
for name in ('two-hosts','blank-second-host'):
 rr=copy.deepcopy(r)
 if name=='blank-second-host':
  run=rr.find('.//hp:p[@id="9032"]/hp:run',ns);run.remove(run.find('hp:pic',ns))
 for parent in rr.iter():
  for n in list(parent):
   if n.tag.endswith('}linesegarray'):parent.remove(n)
 with zipfile.ZipFile(out/(name+'.hwpx'),'w') as dst:
  for i,b in entries:dst.writestr(copy.copy(i),E.tostring(rr,encoding='utf-8',xml_declaration=True) if i.filename=='Contents/section0.xml' else b)
