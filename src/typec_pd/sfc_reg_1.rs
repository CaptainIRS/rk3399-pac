#[doc = "Register `SFC_REG_1` reader"]
pub type R = crate::R<SfcReg1Spec>;
#[doc = "Register `SFC_REG_1` writer"]
pub type W = crate::W<SfcReg1Spec>;
#[doc = "Field `PADDR_M_0` reader - Bits \\[7:0\\]
of paddr_m output."]
pub type PaddrM0R = crate::FieldReader;
#[doc = "Field `PADDR_M_0` writer - Bits \\[7:0\\]
of paddr_m output."]
pub type PaddrM0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PADDR_M_1` reader - Bits \\[15:8\\]
of paddr_m output."]
pub type PaddrM1R = crate::FieldReader;
#[doc = "Field `PADDR_M_1` writer - Bits \\[15:8\\]
of paddr_m output."]
pub type PaddrM1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWDATA_M_0` reader - Bits \\[7:0\\]
of pwdata_m output."]
pub type PwdataM0R = crate::FieldReader;
#[doc = "Field `PWDATA_M_0` writer - Bits \\[7:0\\]
of pwdata_m output."]
pub type PwdataM0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWDATA_M_1` reader - Bits \\[15:8\\]
of pwdata_m output."]
pub type PwdataM1R = crate::FieldReader;
#[doc = "Field `PWDATA_M_1` writer - Bits \\[15:8\\]
of pwdata_m output."]
pub type PwdataM1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bits \\[7:0\\]
of paddr_m output."]
    #[inline(always)]
    pub fn paddr_m_0(&self) -> PaddrM0R {
        PaddrM0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Bits \\[15:8\\]
of paddr_m output."]
    #[inline(always)]
    pub fn paddr_m_1(&self) -> PaddrM1R {
        PaddrM1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Bits \\[7:0\\]
of pwdata_m output."]
    #[inline(always)]
    pub fn pwdata_m_0(&self) -> PwdataM0R {
        PwdataM0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Bits \\[15:8\\]
of pwdata_m output."]
    #[inline(always)]
    pub fn pwdata_m_1(&self) -> PwdataM1R {
        PwdataM1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bits \\[7:0\\]
of paddr_m output."]
    #[inline(always)]
    #[must_use]
    pub fn paddr_m_0(&mut self) -> PaddrM0W<SfcReg1Spec> {
        PaddrM0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Bits \\[15:8\\]
of paddr_m output."]
    #[inline(always)]
    #[must_use]
    pub fn paddr_m_1(&mut self) -> PaddrM1W<SfcReg1Spec> {
        PaddrM1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Bits \\[7:0\\]
of pwdata_m output."]
    #[inline(always)]
    #[must_use]
    pub fn pwdata_m_0(&mut self) -> PwdataM0W<SfcReg1Spec> {
        PwdataM0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Bits \\[15:8\\]
of pwdata_m output."]
    #[inline(always)]
    #[must_use]
    pub fn pwdata_m_1(&mut self) -> PwdataM1W<SfcReg1Spec> {
        PwdataM1W::new(self, 24)
    }
}
#[doc = "SFC Reg1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfc_reg_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfc_reg_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfcReg1Spec;
impl crate::RegisterSpec for SfcReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfc_reg_1::R`](R) reader structure"]
impl crate::Readable for SfcReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfc_reg_1::W`](W) writer structure"]
impl crate::Writable for SfcReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SFC_REG_1 to value 0"]
impl crate::Resettable for SfcReg1Spec {
    const RESET_VALUE: u32 = 0;
}
