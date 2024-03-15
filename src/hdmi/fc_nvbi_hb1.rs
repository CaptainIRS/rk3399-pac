#[doc = "Register `FC_NVBI_HB1` reader"]
pub type R = crate::R<FcNvbiHb1Spec>;
#[doc = "Register `FC_NVBI_HB1` writer"]
pub type W = crate::W<FcNvbiHb1Spec>;
#[doc = "Field `FC_NVBI_HB0` reader - Frame Composer NTSC VBI Packet Header Register 1"]
pub type FcNvbiHb0R = crate::FieldReader;
#[doc = "Field `FC_NVBI_HB0` writer - Frame Composer NTSC VBI Packet Header Register 1"]
pub type FcNvbiHb0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer NTSC VBI Packet Header Register 1"]
    #[inline(always)]
    pub fn fc_nvbi_hb0(&self) -> FcNvbiHb0R {
        FcNvbiHb0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer NTSC VBI Packet Header Register 1"]
    #[inline(always)]
    #[must_use]
    pub fn fc_nvbi_hb0(&mut self) -> FcNvbiHb0W<FcNvbiHb1Spec> {
        FcNvbiHb0W::new(self, 0)
    }
}
#[doc = "Frame Composer NTSC VBI Packet Header Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_nvbi_hb1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_nvbi_hb1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcNvbiHb1Spec;
impl crate::RegisterSpec for FcNvbiHb1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_nvbi_hb1::R`](R) reader structure"]
impl crate::Readable for FcNvbiHb1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_nvbi_hb1::W`](W) writer structure"]
impl crate::Writable for FcNvbiHb1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_NVBI_HB1 to value 0"]
impl crate::Resettable for FcNvbiHb1Spec {
    const RESET_VALUE: u8 = 0;
}
