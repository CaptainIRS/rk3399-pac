#[doc = "Register `DDR_DENALI_CTL_197` reader"]
pub type R = crate::R<DdrDenaliCtl197Spec>;
#[doc = "Register `DDR_DENALI_CTL_197` writer"]
pub type W = crate::W<DdrDenaliCtl197Spec>;
#[doc = "Field `MEMDATA_RATIO_0` reader - Defines the ratio of the DRAM device size on chip select 0 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
pub type MemdataRatio0R = crate::FieldReader;
#[doc = "Field `MEMDATA_RATIO_0` writer - Defines the ratio of the DRAM device size on chip select 0 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
pub type MemdataRatio0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Defines the ratio of the DRAM device size on chip select 0 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
    #[inline(always)]
    pub fn memdata_ratio_0(&self) -> MemdataRatio0R {
        MemdataRatio0R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Defines the ratio of the DRAM device size on chip select 0 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
    #[inline(always)]
    #[must_use]
    pub fn memdata_ratio_0(&mut self) -> MemdataRatio0W<DdrDenaliCtl197Spec> {
        MemdataRatio0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_197::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_197::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl197Spec;
impl crate::RegisterSpec for DdrDenaliCtl197Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_197::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl197Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_197::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl197Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_197 to value 0"]
impl crate::Resettable for DdrDenaliCtl197Spec {
    const RESET_VALUE: u32 = 0;
}
