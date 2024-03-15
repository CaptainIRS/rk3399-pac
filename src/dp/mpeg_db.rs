#[doc = "Register `MPEG_DB%s` reader"]
pub type R = crate::R<MpegDbSpec>;
#[doc = "Register `MPEG_DB%s` writer"]
pub type W = crate::W<MpegDbSpec>;
#[doc = "Field `MPEG_DB1_MPEG_DB10` reader - MPEG InfoFrame Data Byte 1 ~ 10"]
pub type MpegDb1MpegDb10R = crate::FieldReader;
#[doc = "Field `MPEG_DB1_MPEG_DB10` writer - MPEG InfoFrame Data Byte 1 ~ 10"]
pub type MpegDb1MpegDb10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MPEG InfoFrame Data Byte 1 ~ 10"]
    #[inline(always)]
    pub fn mpeg_db1_mpeg_db10(&self) -> MpegDb1MpegDb10R {
        MpegDb1MpegDb10R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MPEG InfoFrame Data Byte 1 ~ 10"]
    #[inline(always)]
    #[must_use]
    pub fn mpeg_db1_mpeg_db10(&mut self) -> MpegDb1MpegDb10W<MpegDbSpec> {
        MpegDb1MpegDb10W::new(self, 0)
    }
}
#[doc = "MPEG Source InfoFrame Packet Data Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpeg_db::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpeg_db::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpegDbSpec;
impl crate::RegisterSpec for MpegDbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpeg_db::R`](R) reader structure"]
impl crate::Readable for MpegDbSpec {}
#[doc = "`write(|w| ..)` method takes [`mpeg_db::W`](W) writer structure"]
impl crate::Writable for MpegDbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPEG_DB%s to value 0"]
impl crate::Resettable for MpegDbSpec {
    const RESET_VALUE: u32 = 0;
}
