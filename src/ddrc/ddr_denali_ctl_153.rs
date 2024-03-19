#[doc = "Register `DDR_DENALI_CTL_153` reader"]
pub type R = crate::R<DdrDenaliCtl153Spec>;
#[doc = "Register `DDR_DENALI_CTL_153` writer"]
pub type W = crate::W<DdrDenaliCtl153Spec>;
#[doc = "Field `MR3_DATA_F2_1` reader - Data to program into memory mode register 3 for chip select 1 for frequency copy 2."]
pub type Mr3DataF2_1R = crate::FieldReader<u16>;
#[doc = "Field `MR3_DATA_F2_1` writer - Data to program into memory mode register 3 for chip select 1 for frequency copy 2."]
pub type Mr3DataF2_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MR8_DATA_1` reader - Data read from MR8 for chip select 1. READ-ONLY"]
pub type Mr8Data1R = crate::FieldReader;
#[doc = "Field `MR11_DATA_F0_1` reader - Data to program into memory mode register 11 for chip select 1 for frequency copy 0."]
pub type Mr11DataF0_1R = crate::FieldReader;
#[doc = "Field `MR11_DATA_F0_1` writer - Data to program into memory mode register 11 for chip select 1 for frequency copy 0."]
pub type Mr11DataF0_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Data to program into memory mode register 3 for chip select 1 for frequency copy 2."]
    #[inline(always)]
    pub fn mr3_data_f2_1(&self) -> Mr3DataF2_1R {
        Mr3DataF2_1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Data read from MR8 for chip select 1. READ-ONLY"]
    #[inline(always)]
    pub fn mr8_data_1(&self) -> Mr8Data1R {
        Mr8Data1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data to program into memory mode register 11 for chip select 1 for frequency copy 0."]
    #[inline(always)]
    pub fn mr11_data_f0_1(&self) -> Mr11DataF0_1R {
        Mr11DataF0_1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data to program into memory mode register 3 for chip select 1 for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn mr3_data_f2_1(&mut self) -> Mr3DataF2_1W<DdrDenaliCtl153Spec> {
        Mr3DataF2_1W::new(self, 0)
    }
    #[doc = "Bits 24:31 - Data to program into memory mode register 11 for chip select 1 for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr11_data_f0_1(&mut self) -> Mr11DataF0_1W<DdrDenaliCtl153Spec> {
        Mr11DataF0_1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_153::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_153::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl153Spec;
impl crate::RegisterSpec for DdrDenaliCtl153Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_153::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl153Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_153::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl153Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_153 to value 0"]
impl crate::Resettable for DdrDenaliCtl153Spec {
    const RESET_VALUE: u32 = 0;
}
