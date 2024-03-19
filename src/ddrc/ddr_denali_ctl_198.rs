#[doc = "Register `DDR_DENALI_CTL_198` reader"]
pub type R = crate::R<DdrDenaliCtl198Spec>;
#[doc = "Register `DDR_DENALI_CTL_198` writer"]
pub type W = crate::W<DdrDenaliCtl198Spec>;
#[doc = "Field `MEMDATA_RATIO_1` reader - Defines the ratio of the DRAM device size on chip select 1 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
pub type MemdataRatio1R = crate::FieldReader;
#[doc = "Field `MEMDATA_RATIO_1` writer - Defines the ratio of the DRAM device size on chip select 1 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
pub type MemdataRatio1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 8:10 - Defines the ratio of the DRAM device size on chip select 1 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
    #[inline(always)]
    pub fn memdata_ratio_1(&self) -> MemdataRatio1R {
        MemdataRatio1R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Defines the ratio of the DRAM device size on chip select 1 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
    #[inline(always)]
    #[must_use]
    pub fn memdata_ratio_1(&mut self) -> MemdataRatio1W<DdrDenaliCtl198Spec> {
        MemdataRatio1W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_198::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_198::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl198Spec;
impl crate::RegisterSpec for DdrDenaliCtl198Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_198::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl198Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_198::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl198Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_198 to value 0"]
impl crate::Resettable for DdrDenaliCtl198Spec {
    const RESET_VALUE: u32 = 0;
}
