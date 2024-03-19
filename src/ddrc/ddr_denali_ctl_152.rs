#[doc = "Register `DDR_DENALI_CTL_152` reader"]
pub type R = crate::R<DdrDenaliCtl152Spec>;
#[doc = "Register `DDR_DENALI_CTL_152` writer"]
pub type W = crate::W<DdrDenaliCtl152Spec>;
#[doc = "Field `MR3_DATA_F0_1` reader - Data to program into memory mode register 3 for chip select 1 for frequency copy 0."]
pub type Mr3DataF0_1R = crate::FieldReader<u16>;
#[doc = "Field `MR3_DATA_F0_1` writer - Data to program into memory mode register 3 for chip select 1 for frequency copy 0."]
pub type Mr3DataF0_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MR3_DATA_F1_1` reader - Data to program into memory mode register 3 for chip select 1 for frequency copy 1."]
pub type Mr3DataF1_1R = crate::FieldReader<u16>;
#[doc = "Field `MR3_DATA_F1_1` writer - Data to program into memory mode register 3 for chip select 1 for frequency copy 1."]
pub type Mr3DataF1_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data to program into memory mode register 3 for chip select 1 for frequency copy 0."]
    #[inline(always)]
    pub fn mr3_data_f0_1(&self) -> Mr3DataF0_1R {
        Mr3DataF0_1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 3 for chip select 1 for frequency copy 1."]
    #[inline(always)]
    pub fn mr3_data_f1_1(&self) -> Mr3DataF1_1R {
        Mr3DataF1_1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data to program into memory mode register 3 for chip select 1 for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr3_data_f0_1(&mut self) -> Mr3DataF0_1W<DdrDenaliCtl152Spec> {
        Mr3DataF0_1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 3 for chip select 1 for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn mr3_data_f1_1(&mut self) -> Mr3DataF1_1W<DdrDenaliCtl152Spec> {
        Mr3DataF1_1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_152::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_152::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl152Spec;
impl crate::RegisterSpec for DdrDenaliCtl152Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_152::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl152Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_152::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl152Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_152 to value 0"]
impl crate::Resettable for DdrDenaliCtl152Spec {
    const RESET_VALUE: u32 = 0;
}
