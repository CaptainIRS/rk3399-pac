#[doc = "Register `DENALI_CTL_154` reader"]
pub type R = crate::R<DenaliCtl154Spec>;
#[doc = "Register `DENALI_CTL_154` writer"]
pub type W = crate::W<DenaliCtl154Spec>;
#[doc = "Field `MR11_DATA_F1_1` reader - Data to program into memory mode register 11 for chip select 1 for frequency copy 1."]
pub type Mr11DataF1_1R = crate::FieldReader;
#[doc = "Field `MR11_DATA_F1_1` writer - Data to program into memory mode register 11 for chip select 1 for frequency copy 1."]
pub type Mr11DataF1_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MR11_DATA_F2_1` reader - Data to program into memory mode register 11 for chip select 1 for frequency copy 2."]
pub type Mr11DataF2_1R = crate::FieldReader;
#[doc = "Field `MR11_DATA_F2_1` writer - Data to program into memory mode register 11 for chip select 1 for frequency copy 2."]
pub type Mr11DataF2_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MR12_DATA_F0_1` reader - Data to program into memory mode register 12 for chip select 1."]
pub type Mr12DataF0_1R = crate::FieldReader<u16>;
#[doc = "Field `MR12_DATA_F0_1` writer - Data to program into memory mode register 12 for chip select 1."]
pub type Mr12DataF0_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Data to program into memory mode register 11 for chip select 1 for frequency copy 1."]
    #[inline(always)]
    pub fn mr11_data_f1_1(&self) -> Mr11DataF1_1R {
        Mr11DataF1_1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data to program into memory mode register 11 for chip select 1 for frequency copy 2."]
    #[inline(always)]
    pub fn mr11_data_f2_1(&self) -> Mr11DataF2_1R {
        Mr11DataF2_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 12 for chip select 1."]
    #[inline(always)]
    pub fn mr12_data_f0_1(&self) -> Mr12DataF0_1R {
        Mr12DataF0_1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data to program into memory mode register 11 for chip select 1 for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn mr11_data_f1_1(&mut self) -> Mr11DataF1_1W<DenaliCtl154Spec> {
        Mr11DataF1_1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data to program into memory mode register 11 for chip select 1 for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn mr11_data_f2_1(&mut self) -> Mr11DataF2_1W<DenaliCtl154Spec> {
        Mr11DataF2_1W::new(self, 8)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 12 for chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn mr12_data_f0_1(&mut self) -> Mr12DataF0_1W<DenaliCtl154Spec> {
        Mr12DataF0_1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_154::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_154::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl154Spec;
impl crate::RegisterSpec for DenaliCtl154Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_154::R`](R) reader structure"]
impl crate::Readable for DenaliCtl154Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_154::W`](W) writer structure"]
impl crate::Writable for DenaliCtl154Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_154 to value 0"]
impl crate::Resettable for DenaliCtl154Spec {
    const RESET_VALUE: u32 = 0;
}
