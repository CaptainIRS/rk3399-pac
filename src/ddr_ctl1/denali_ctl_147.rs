#[doc = "Register `DENALI_CTL_147` reader"]
pub type R = crate::R<DenaliCtl147Spec>;
#[doc = "Register `DENALI_CTL_147` writer"]
pub type W = crate::W<DenaliCtl147Spec>;
#[doc = "Field `MR0_DATA_F0_1` reader - Data to program into memory mode register 0 for chip select 1 for frequency copy 0."]
pub type Mr0DataF0_1R = crate::FieldReader<u16>;
#[doc = "Field `MR0_DATA_F0_1` writer - Data to program into memory mode register 0 for chip select 1 for frequency copy 0."]
pub type Mr0DataF0_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MR1_DATA_F0_1` reader - Data to program into memory mode register 1 for chip select 1 for frequency copy 0."]
pub type Mr1DataF0_1R = crate::FieldReader<u16>;
#[doc = "Field `MR1_DATA_F0_1` writer - Data to program into memory mode register 1 for chip select 1 for frequency copy 0."]
pub type Mr1DataF0_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data to program into memory mode register 0 for chip select 1 for frequency copy 0."]
    #[inline(always)]
    pub fn mr0_data_f0_1(&self) -> Mr0DataF0_1R {
        Mr0DataF0_1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 1 for chip select 1 for frequency copy 0."]
    #[inline(always)]
    pub fn mr1_data_f0_1(&self) -> Mr1DataF0_1R {
        Mr1DataF0_1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data to program into memory mode register 0 for chip select 1 for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr0_data_f0_1(&mut self) -> Mr0DataF0_1W<DenaliCtl147Spec> {
        Mr0DataF0_1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 1 for chip select 1 for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr1_data_f0_1(&mut self) -> Mr1DataF0_1W<DenaliCtl147Spec> {
        Mr1DataF0_1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_147::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_147::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl147Spec;
impl crate::RegisterSpec for DenaliCtl147Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_147::R`](R) reader structure"]
impl crate::Readable for DenaliCtl147Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_147::W`](W) writer structure"]
impl crate::Writable for DenaliCtl147Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_147 to value 0"]
impl crate::Resettable for DenaliCtl147Spec {
    const RESET_VALUE: u32 = 0;
}
