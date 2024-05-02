#[doc = "Register `FILT_FAC_MID` reader"]
pub type R = crate::R<FiltFacMidSpec>;
#[doc = "Register `FILT_FAC_MID` writer"]
pub type W = crate::W<FiltFacMidSpec>;
#[doc = "Field `filt_fac_mid` reader - Filter factor for mid level and for static filter mode\n\n"]
pub type FiltFacMidR = crate::FieldReader;
#[doc = "Field `filt_fac_mid` writer - Filter factor for mid level and for static filter mode\n\n"]
pub type FiltFacMidW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Filter factor for mid level and for static filter mode\n\n"]
    #[inline(always)]
    pub fn filt_fac_mid(&self) -> FiltFacMidR {
        FiltFacMidR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Filter factor for mid level and for static filter mode\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn filt_fac_mid(&mut self) -> FiltFacMidW<FiltFacMidSpec> {
        FiltFacMidW::new(self, 0)
    }
}
#[doc = "filter factor middle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_fac_mid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_fac_mid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiltFacMidSpec;
impl crate::RegisterSpec for FiltFacMidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filt_fac_mid::R`](R) reader structure"]
impl crate::Readable for FiltFacMidSpec {}
#[doc = "`write(|w| ..)` method takes [`filt_fac_mid::W`](W) writer structure"]
impl crate::Writable for FiltFacMidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILT_FAC_MID to value 0x0a"]
impl crate::Resettable for FiltFacMidSpec {
    const RESET_VALUE: u32 = 0x0a;
}
