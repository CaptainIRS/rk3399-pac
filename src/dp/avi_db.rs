#[doc = "Register `AVI_DB%s` reader"]
pub type R = crate::R<AviDbSpec>;
#[doc = "Register `AVI_DB%s` writer"]
pub type W = crate::W<AviDbSpec>;
#[doc = "Field `AVI_DB1_AVI_DB13` reader - AVI Data Byte 1 ~ 13"]
pub type AviDb1AviDb13R = crate::FieldReader;
#[doc = "Field `AVI_DB1_AVI_DB13` writer - AVI Data Byte 1 ~ 13"]
pub type AviDb1AviDb13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - AVI Data Byte 1 ~ 13"]
    #[inline(always)]
    pub fn avi_db1_avi_db13(&self) -> AviDb1AviDb13R {
        AviDb1AviDb13R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AVI Data Byte 1 ~ 13"]
    #[inline(always)]
    #[must_use]
    pub fn avi_db1_avi_db13(&mut self) -> AviDb1AviDb13W<AviDbSpec> {
        AviDb1AviDb13W::new(self, 0)
    }
}
#[doc = "AVI InfoFrame Packet Data Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`avi_db::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`avi_db::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AviDbSpec;
impl crate::RegisterSpec for AviDbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`avi_db::R`](R) reader structure"]
impl crate::Readable for AviDbSpec {}
#[doc = "`write(|w| ..)` method takes [`avi_db::W`](W) writer structure"]
impl crate::Writable for AviDbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AVI_DB%s to value 0"]
impl crate::Resettable for AviDbSpec {
    const RESET_VALUE: u32 = 0;
}
