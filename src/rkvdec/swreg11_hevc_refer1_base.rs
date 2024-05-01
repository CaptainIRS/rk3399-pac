#[doc = "Register `SWREG11_HEVC_REFER1_BASE` reader"]
pub type R = crate::R<Swreg11HevcRefer1BaseSpec>;
#[doc = "Register `SWREG11_HEVC_REFER1_BASE` writer"]
pub type W = crate::W<Swreg11HevcRefer1BaseSpec>;
#[doc = "Field `SW_REF_VALID_4_7` reader - valid flag for picture index 4 ~7\n\nvalid flag for picture index 4 ~7"]
pub type SwRefValid4_7R = crate::FieldReader;
#[doc = "Field `SW_REF_VALID_4_7` writer - valid flag for picture index 4 ~7\n\nvalid flag for picture index 4 ~7"]
pub type SwRefValid4_7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SW_REFER1_BASE` reader - base address for reference picture index 1\n\nbase address for reference picture index 1 (the address should be\n\n128bit align)"]
pub type SwRefer1BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER1_BASE` writer - base address for reference picture index 1\n\nbase address for reference picture index 1 (the address should be\n\n128bit align)"]
pub type SwRefer1BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - valid flag for picture index 4 ~7\n\nvalid flag for picture index 4 ~7"]
    #[inline(always)]
    pub fn sw_ref_valid_4_7(&self) -> SwRefValid4_7R {
        SwRefValid4_7R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - base address for reference picture index 1\n\nbase address for reference picture index 1 (the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_refer1_base(&self) -> SwRefer1BaseR {
        SwRefer1BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - valid flag for picture index 4 ~7\n\nvalid flag for picture index 4 ~7"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref_valid_4_7(&mut self) -> SwRefValid4_7W<Swreg11HevcRefer1BaseSpec> {
        SwRefValid4_7W::new(self, 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index 1\n\nbase address for reference picture index 1 (the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer1_base(&mut self) -> SwRefer1BaseW<Swreg11HevcRefer1BaseSpec> {
        SwRefer1BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg11_hevc_refer1_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg11_hevc_refer1_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg11HevcRefer1BaseSpec;
impl crate::RegisterSpec for Swreg11HevcRefer1BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg11_hevc_refer1_base::R`](R) reader structure"]
impl crate::Readable for Swreg11HevcRefer1BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg11_hevc_refer1_base::W`](W) writer structure"]
impl crate::Writable for Swreg11HevcRefer1BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG11_HEVC_REFER1_BASE to value 0"]
impl crate::Resettable for Swreg11HevcRefer1BaseSpec {
    const RESET_VALUE: u32 = 0;
}
