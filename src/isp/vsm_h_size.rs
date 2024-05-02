#[doc = "Register `VSM_H_SIZE` reader"]
pub type R = crate::R<VsmHSizeSpec>;
#[doc = "Register `VSM_H_SIZE` writer"]
pub type W = crate::W<VsmHSizeSpec>;
#[doc = "Field `vsm_h_size` reader - Horizontal size in pixels."]
pub type VsmHSizeR = crate::FieldReader<u16>;
#[doc = "Field `vsm_h_size` writer - Horizontal size in pixels."]
pub type VsmHSizeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Horizontal size in pixels."]
    #[inline(always)]
    pub fn vsm_h_size(&self) -> VsmHSizeR {
        VsmHSizeR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Horizontal size in pixels."]
    #[inline(always)]
    #[must_use]
    pub fn vsm_h_size(&mut self) -> VsmHSizeW<VsmHSizeSpec> {
        VsmHSizeW::new(self, 1)
    }
}
#[doc = "Horizontal measure window size\n\nNote: only even values are allowed: vsm_h_size\\[0\\]
not writable and read returns 0. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsm_h_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsm_h_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VsmHSizeSpec;
impl crate::RegisterSpec for VsmHSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vsm_h_size::R`](R) reader structure"]
impl crate::Readable for VsmHSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`vsm_h_size::W`](W) writer structure"]
impl crate::Writable for VsmHSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VSM_H_SIZE to value 0"]
impl crate::Resettable for VsmHSizeSpec {
    const RESET_VALUE: u32 = 0;
}
