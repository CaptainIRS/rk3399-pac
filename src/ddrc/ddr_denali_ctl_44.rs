#[doc = "Register `DDR_DENALI_CTL_44` reader"]
pub type R = crate::R<DdrDenaliCtl44Spec>;
#[doc = "Register `DDR_DENALI_CTL_44` writer"]
pub type W = crate::W<DdrDenaliCtl44Spec>;
#[doc = "Field `TDAL_F0` reader - DRAM TDAL value for frequency copy 0 in cycles."]
pub type TdalF0R = crate::FieldReader;
#[doc = "Field `TDAL_F0` writer - DRAM TDAL value for frequency copy 0 in cycles."]
pub type TdalF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDAL_F1` reader - DRAM TDAL value for frequency copy 1 in cycles."]
pub type TdalF1R = crate::FieldReader;
#[doc = "Field `TDAL_F1` writer - DRAM TDAL value for frequency copy 1 in cycles."]
pub type TdalF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDAL_F2` reader - DRAM TDAL value for frequency copy 2 in cycles."]
pub type TdalF2R = crate::FieldReader;
#[doc = "Field `TDAL_F2` writer - DRAM TDAL value for frequency copy 2 in cycles."]
pub type TdalF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BSTLEN` reader - Encoded burst length sent to DRAMs during initialization. Program to 1 for BL2, program to 2 for BL4, or program to 3 for BL8."]
pub type BstlenR = crate::FieldReader;
#[doc = "Field `BSTLEN` writer - Encoded burst length sent to DRAMs during initialization. Program to 1 for BL2, program to 2 for BL4, or program to 3 for BL8."]
pub type BstlenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - DRAM TDAL value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tdal_f0(&self) -> TdalF0R {
        TdalF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DRAM TDAL value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tdal_f1(&self) -> TdalF1R {
        TdalF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DRAM TDAL value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tdal_f2(&self) -> TdalF2R {
        TdalF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - Encoded burst length sent to DRAMs during initialization. Program to 1 for BL2, program to 2 for BL4, or program to 3 for BL8."]
    #[inline(always)]
    pub fn bstlen(&self) -> BstlenR {
        BstlenR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DRAM TDAL value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tdal_f0(&mut self) -> TdalF0W<DdrDenaliCtl44Spec> {
        TdalF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TDAL value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tdal_f1(&mut self) -> TdalF1W<DdrDenaliCtl44Spec> {
        TdalF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DRAM TDAL value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tdal_f2(&mut self) -> TdalF2W<DdrDenaliCtl44Spec> {
        TdalF2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Encoded burst length sent to DRAMs during initialization. Program to 1 for BL2, program to 2 for BL4, or program to 3 for BL8."]
    #[inline(always)]
    #[must_use]
    pub fn bstlen(&mut self) -> BstlenW<DdrDenaliCtl44Spec> {
        BstlenW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl44Spec;
impl crate::RegisterSpec for DdrDenaliCtl44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_44::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl44Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_44::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_44 to value 0x0200_0000"]
impl crate::Resettable for DdrDenaliCtl44Spec {
    const RESET_VALUE: u32 = 0x0200_0000;
}
