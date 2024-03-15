#[doc = "Register `FC_NVBI_HB2` reader"]
pub type R = crate::R<FcNvbiHb2Spec>;
#[doc = "Register `FC_NVBI_HB2` writer"]
pub type W = crate::W<FcNvbiHb2Spec>;
#[doc = "Field `FC_NVBI_HB1` reader - Frame Composer NTSC VBI Packet Header Register 2"]
pub type FcNvbiHb1R = crate::FieldReader;
#[doc = "Field `FC_NVBI_HB1` writer - Frame Composer NTSC VBI Packet Header Register 2"]
pub type FcNvbiHb1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer NTSC VBI Packet Header Register 2"]
    #[inline(always)]
    pub fn fc_nvbi_hb1(&self) -> FcNvbiHb1R {
        FcNvbiHb1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer NTSC VBI Packet Header Register 2"]
    #[inline(always)]
    #[must_use]
    pub fn fc_nvbi_hb1(&mut self) -> FcNvbiHb1W<FcNvbiHb2Spec> {
        FcNvbiHb1W::new(self, 0)
    }
}
#[doc = "Frame Composer NTSC VBI Packet Header Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_nvbi_hb2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_nvbi_hb2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcNvbiHb2Spec;
impl crate::RegisterSpec for FcNvbiHb2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_nvbi_hb2::R`](R) reader structure"]
impl crate::Readable for FcNvbiHb2Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_nvbi_hb2::W`](W) writer structure"]
impl crate::Writable for FcNvbiHb2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_NVBI_HB2 to value 0"]
impl crate::Resettable for FcNvbiHb2Spec {
    const RESET_VALUE: u8 = 0;
}
