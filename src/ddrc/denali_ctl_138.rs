#[doc = "Register `DENALI_CTL_138` reader"]
pub type R = crate::R<DenaliCtl138Spec>;
#[doc = "Register `DENALI_CTL_138` writer"]
pub type W = crate::W<DenaliCtl138Spec>;
#[doc = "Field `MR3_DATA_F0_0` reader - Data to program into memory mode register 3 for chip select 0 for frequency copy 0."]
pub type Mr3DataF0_0R = crate::FieldReader<u16>;
#[doc = "Field `MR3_DATA_F0_0` writer - Data to program into memory mode register 3 for chip select 0 for frequency copy 0."]
pub type Mr3DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MR3_DATA_F1_0` reader - Data to program into memory mode register 3 for chip select 0 for frequency copy 1."]
pub type Mr3DataF1_0R = crate::FieldReader<u16>;
#[doc = "Field `MR3_DATA_F1_0` writer - Data to program into memory mode register 3 for chip select 0 for frequency copy 1."]
pub type Mr3DataF1_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data to program into memory mode register 3 for chip select 0 for frequency copy 0."]
    #[inline(always)]
    pub fn mr3_data_f0_0(&self) -> Mr3DataF0_0R {
        Mr3DataF0_0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 3 for chip select 0 for frequency copy 1."]
    #[inline(always)]
    pub fn mr3_data_f1_0(&self) -> Mr3DataF1_0R {
        Mr3DataF1_0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data to program into memory mode register 3 for chip select 0 for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr3_data_f0_0(&mut self) -> Mr3DataF0_0W<DenaliCtl138Spec> {
        Mr3DataF0_0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Data to program into memory mode register 3 for chip select 0 for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn mr3_data_f1_0(&mut self) -> Mr3DataF1_0W<DenaliCtl138Spec> {
        Mr3DataF1_0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_138::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_138::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl138Spec;
impl crate::RegisterSpec for DenaliCtl138Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_138::R`](R) reader structure"]
impl crate::Readable for DenaliCtl138Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_138::W`](W) writer structure"]
impl crate::Writable for DenaliCtl138Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_138 to value 0"]
impl crate::Resettable for DenaliCtl138Spec {
    const RESET_VALUE: u32 = 0;
}
