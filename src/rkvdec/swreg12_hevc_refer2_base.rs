#[doc = "Register `SWREG12_HEVC_REFER2_BASE` reader"]
pub type R = crate::R<Swreg12HevcRefer2BaseSpec>;
#[doc = "Register `SWREG12_HEVC_REFER2_BASE` writer"]
pub type W = crate::W<Swreg12HevcRefer2BaseSpec>;
#[doc = "Field `SW_REF_VALID_8_11` reader - valid flag for picture index 8~11\n\nvalid flag for picture index 8~11"]
pub type SwRefValid8_11R = crate::FieldReader;
#[doc = "Field `SW_REF_VALID_8_11` writer - valid flag for picture index 8~11\n\nvalid flag for picture index 8~11"]
pub type SwRefValid8_11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SW_REFER2_BASE` reader - base address for reference picture index 2\n\nbase address for reference picture index 2 (the address should be\n\n128bit align)"]
pub type SwRefer2BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER2_BASE` writer - base address for reference picture index 2\n\nbase address for reference picture index 2 (the address should be\n\n128bit align)"]
pub type SwRefer2BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - valid flag for picture index 8~11\n\nvalid flag for picture index 8~11"]
    #[inline(always)]
    pub fn sw_ref_valid_8_11(&self) -> SwRefValid8_11R {
        SwRefValid8_11R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - base address for reference picture index 2\n\nbase address for reference picture index 2 (the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_refer2_base(&self) -> SwRefer2BaseR {
        SwRefer2BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - valid flag for picture index 8~11\n\nvalid flag for picture index 8~11"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ref_valid_8_11(&mut self) -> SwRefValid8_11W<Swreg12HevcRefer2BaseSpec> {
        SwRefValid8_11W::new(self, 0)
    }
    #[doc = "Bits 4:31 - base address for reference picture index 2\n\nbase address for reference picture index 2 (the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer2_base(&mut self) -> SwRefer2BaseW<Swreg12HevcRefer2BaseSpec> {
        SwRefer2BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg12_hevc_refer2_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg12_hevc_refer2_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg12HevcRefer2BaseSpec;
impl crate::RegisterSpec for Swreg12HevcRefer2BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg12_hevc_refer2_base::R`](R) reader structure"]
impl crate::Readable for Swreg12HevcRefer2BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg12_hevc_refer2_base::W`](W) writer structure"]
impl crate::Writable for Swreg12HevcRefer2BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG12_HEVC_REFER2_BASE to value 0"]
impl crate::Resettable for Swreg12HevcRefer2BaseSpec {
    const RESET_VALUE: u32 = 0;
}
