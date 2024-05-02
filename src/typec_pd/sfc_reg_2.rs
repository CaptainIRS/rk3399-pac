#[doc = "Register `SFC_REG_2` reader"]
pub type R = crate::R<SfcReg2Spec>;
#[doc = "Field `PRDATA_M_0` reader - Bits \\[7:0\\]
of prdata_m input."]
pub type PrdataM0R = crate::FieldReader;
#[doc = "Field `PRDATA_M_1` reader - Bits \\[15:8\\]
of prdata_m input."]
pub type PrdataM1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bits \\[7:0\\]
of prdata_m input."]
    #[inline(always)]
    pub fn prdata_m_0(&self) -> PrdataM0R {
        PrdataM0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Bits \\[15:8\\]
of prdata_m input."]
    #[inline(always)]
    pub fn prdata_m_1(&self) -> PrdataM1R {
        PrdataM1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "SFC Reg2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfc_reg_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfcReg2Spec;
impl crate::RegisterSpec for SfcReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfc_reg_2::R`](R) reader structure"]
impl crate::Readable for SfcReg2Spec {}
#[doc = "`reset()` method sets SFC_REG_2 to value 0x0002_0000"]
impl crate::Resettable for SfcReg2Spec {
    const RESET_VALUE: u32 = 0x0002_0000;
}
