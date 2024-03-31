#[doc = "Register `DENALI_CTL_139` reader"]
pub type R = crate::R<DenaliCtl139Spec>;
#[doc = "Register `DENALI_CTL_139` writer"]
pub type W = crate::W<DenaliCtl139Spec>;
#[doc = "Field `MR3_DATA_F2_0` reader - Data to program into memory mode register 3 for chip select 0 for frequency copy 2."]
pub type Mr3DataF2_0R = crate::FieldReader<u16>;
#[doc = "Field `MR3_DATA_F2_0` writer - Data to program into memory mode register 3 for chip select 0 for frequency copy 2."]
pub type Mr3DataF2_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MR8_DATA_0` reader - Data read from MR8 for chip select 0."]
pub type Mr8Data0R = crate::FieldReader;
#[doc = "Field `MR11_DATA_F0_0` reader - Data to program into memory mode register 11 for chip select 0 for frequency copy 0."]
pub type Mr11DataF0_0R = crate::FieldReader;
#[doc = "Field `MR11_DATA_F0_0` writer - Data to program into memory mode register 11 for chip select 0 for frequency copy 0."]
pub type Mr11DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Data to program into memory mode register 3 for chip select 0 for frequency copy 2."]
    #[inline(always)]
    pub fn mr3_data_f2_0(&self) -> Mr3DataF2_0R {
        Mr3DataF2_0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Data read from MR8 for chip select 0."]
    #[inline(always)]
    pub fn mr8_data_0(&self) -> Mr8Data0R {
        Mr8Data0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data to program into memory mode register 11 for chip select 0 for frequency copy 0."]
    #[inline(always)]
    pub fn mr11_data_f0_0(&self) -> Mr11DataF0_0R {
        Mr11DataF0_0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data to program into memory mode register 3 for chip select 0 for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn mr3_data_f2_0(&mut self) -> Mr3DataF2_0W<DenaliCtl139Spec> {
        Mr3DataF2_0W::new(self, 0)
    }
    #[doc = "Bits 24:31 - Data to program into memory mode register 11 for chip select 0 for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr11_data_f0_0(&mut self) -> Mr11DataF0_0W<DenaliCtl139Spec> {
        Mr11DataF0_0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_139::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_139::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl139Spec;
impl crate::RegisterSpec for DenaliCtl139Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_139::R`](R) reader structure"]
impl crate::Readable for DenaliCtl139Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_139::W`](W) writer structure"]
impl crate::Writable for DenaliCtl139Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_139 to value 0"]
impl crate::Resettable for DenaliCtl139Spec {
    const RESET_VALUE: u32 = 0;
}
