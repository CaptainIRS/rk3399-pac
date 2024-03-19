#[doc = "Register `DDR_DENALI_CTL_136` reader"]
pub type R = crate::R<DdrDenaliCtl136Spec>;
#[doc = "Register `DDR_DENALI_CTL_136` writer"]
pub type W = crate::W<DdrDenaliCtl136Spec>;
#[doc = "Field `MR0_DATA_F2_0` reader - Data to program into memory mode register 0 for chip select 0 for frequency copy 2."]
pub type Mr0DataF2_0R = crate::FieldReader<u16>;
#[doc = "Field `MR0_DATA_F2_0` writer - Data to program into memory mode register 0 for chip select 0 for frequency copy 2."]
pub type Mr0DataF2_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MR1_DATA_F2_0` reader - Data to program into memory mode register 1 for chip select 0 for frequency copy 2."]
pub type Mr1DataF2_0R = crate::FieldReader<u16>;
#[doc = "Field `MR1_DATA_F2_0` writer - Data to program into memory mode register 1 for chip select 0 for frequency copy 2."]
pub type Mr1DataF2_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data to program into memory mode register 0 for chip select 0 for frequency copy 2."]
    #[inline(always)]
    pub fn mr0_data_f2_0(&self) -> Mr0DataF2_0R {
        Mr0DataF2_0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 1 for chip select 0 for frequency copy 2."]
    #[inline(always)]
    pub fn mr1_data_f2_0(&self) -> Mr1DataF2_0R {
        Mr1DataF2_0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data to program into memory mode register 0 for chip select 0 for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn mr0_data_f2_0(&mut self) -> Mr0DataF2_0W<DdrDenaliCtl136Spec> {
        Mr0DataF2_0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 1 for chip select 0 for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn mr1_data_f2_0(&mut self) -> Mr1DataF2_0W<DdrDenaliCtl136Spec> {
        Mr1DataF2_0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_136::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_136::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl136Spec;
impl crate::RegisterSpec for DdrDenaliCtl136Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_136::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl136Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_136::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl136Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_136 to value 0"]
impl crate::Resettable for DdrDenaliCtl136Spec {
    const RESET_VALUE: u32 = 0;
}
