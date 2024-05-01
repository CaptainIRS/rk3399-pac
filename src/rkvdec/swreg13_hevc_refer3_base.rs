#[doc = "Register `SWREG13_HEVC_REFER3_BASE` reader"]
pub type R = crate::R<Swreg13HevcRefer3BaseSpec>;
#[doc = "Register `SWREG13_HEVC_REFER3_BASE` writer"]
pub type W = crate::W<Swreg13HevcRefer3BaseSpec>;
#[doc = "Field `SW_REF_VALID_12_14` reader - valid flag for picture index 12~14\n\nvalid flag for picture index 12~14"]
pub type SwRefValid12_14R = crate::FieldReader;
#[doc = "Field `SW_REF_VALID_12_14` writer - valid flag for picture index 12~14\n\nvalid flag for picture index 12~14"]
pub type SwRefValid12_14W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_REFER3_BASE` reader - base address for reference picture index 3\n\nbase address for reference picture index 3 (the address should be\n\n128bit align)"]
pub type SwRefer3BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER3_BASE` writer - base address for reference picture index 3\n\nbase address for reference picture index 3 (the address should be\n\n128bit align)"]
pub type SwRefer3BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:2 - valid flag for picture index 12~14\n\nvalid flag for picture index 12~14"]
    #[inline(always)]
    pub fn sw_ref_valid_12_14(&self) -> SwRefValid12_14R {
        SwRefValid12_14R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:31 - base address for reference picture index 3\n\nbase address for reference picture index 3 (the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_refer3_base(&self) -> SwRefer3BaseR {
        SwRefer3BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - valid flag for picture index 12~14\n\nvalid flag for picture index 12~14"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref_valid_12_14(&mut self) -> SwRefValid12_14W<Swreg13HevcRefer3BaseSpec> {
        SwRefValid12_14W::new(self, 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index 3\n\nbase address for reference picture index 3 (the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer3_base(&mut self) -> SwRefer3BaseW<Swreg13HevcRefer3BaseSpec> {
        SwRefer3BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg13_hevc_refer3_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg13_hevc_refer3_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg13HevcRefer3BaseSpec;
impl crate::RegisterSpec for Swreg13HevcRefer3BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg13_hevc_refer3_base::R`](R) reader structure"]
impl crate::Readable for Swreg13HevcRefer3BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg13_hevc_refer3_base::W`](W) writer structure"]
impl crate::Writable for Swreg13HevcRefer3BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG13_HEVC_REFER3_BASE to value 0"]
impl crate::Resettable for Swreg13HevcRefer3BaseSpec {
    const RESET_VALUE: u32 = 0;
}
