#[doc = "Register `SWREG10_HEVC_REFER0_BASE` reader"]
pub type R = crate::R<Swreg10HevcRefer0BaseSpec>;
#[doc = "Register `SWREG10_HEVC_REFER0_BASE` writer"]
pub type W = crate::W<Swreg10HevcRefer0BaseSpec>;
#[doc = "Field `SW_REF_VALID_0_3` reader - valid flag for picture index 0 ~3\n\nvalid flag for picture index 0 ~3"]
pub type SwRefValid0_3R = crate::FieldReader;
#[doc = "Field `SW_REF_VALID_0_3` writer - valid flag for picture index 0 ~3\n\nvalid flag for picture index 0 ~3"]
pub type SwRefValid0_3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SW_REFER0_BASE` reader - base address for reference picture index0\n\nbase address for reference picture index 0 (the address should be\n\n128bit align)"]
pub type SwRefer0BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER0_BASE` writer - base address for reference picture index0\n\nbase address for reference picture index 0 (the address should be\n\n128bit align)"]
pub type SwRefer0BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - valid flag for picture index 0 ~3\n\nvalid flag for picture index 0 ~3"]
    #[inline(always)]
    pub fn sw_ref_valid_0_3(&self) -> SwRefValid0_3R {
        SwRefValid0_3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - base address for reference picture index0\n\nbase address for reference picture index 0 (the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_refer0_base(&self) -> SwRefer0BaseR {
        SwRefer0BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - valid flag for picture index 0 ~3\n\nvalid flag for picture index 0 ~3"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref_valid_0_3(&mut self) -> SwRefValid0_3W<Swreg10HevcRefer0BaseSpec> {
        SwRefValid0_3W::new(self, 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index0\n\nbase address for reference picture index 0 (the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer0_base(&mut self) -> SwRefer0BaseW<Swreg10HevcRefer0BaseSpec> {
        SwRefer0BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg10_hevc_refer0_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg10_hevc_refer0_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg10HevcRefer0BaseSpec;
impl crate::RegisterSpec for Swreg10HevcRefer0BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg10_hevc_refer0_base::R`](R) reader structure"]
impl crate::Readable for Swreg10HevcRefer0BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg10_hevc_refer0_base::W`](W) writer structure"]
impl crate::Writable for Swreg10HevcRefer0BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG10_HEVC_REFER0_BASE to value 0"]
impl crate::Resettable for Swreg10HevcRefer0BaseSpec {
    const RESET_VALUE: u32 = 0;
}
