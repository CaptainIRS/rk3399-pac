#[doc = "Register `PI_REG_80` reader"]
pub type R = crate::R<PiReg80Spec>;
#[doc = "Register `PI_REG_80` writer"]
pub type W = crate::W<PiReg80Spec>;
#[doc = "Field `PI_RDLVL_RESP_MASK` reader - Indicates mask for the dfi_rdlvl_resp signal during data eye training."]
pub type PiRdlvlRespMaskR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_RESP_MASK` writer - Indicates mask for the dfi_rdlvl_resp signal during data eye training."]
pub type PiRdlvlRespMaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TDFI_RDLVL_EN` reader - Defines the DFI tRDLVL_EN timing parameter (in DFI clocks), the minimum cycles from a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion to the first read or MRR."]
pub type PiTdfiRdlvlEnR = crate::FieldReader;
#[doc = "Field `PI_TDFI_RDLVL_EN` writer - Defines the DFI tRDLVL_EN timing parameter (in DFI clocks), the minimum cycles from a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion to the first read or MRR."]
pub type PiTdfiRdlvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_RDLVL_EN` reader - Enables the PI data eye training module. Bit1 represents the support when non-initialization. Bit0 represents the support when initialization. Set to 1 to enable."]
pub type PiRdlvlEnR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_EN` writer - Enables the PI data eye training module. Bit1 represents the support when non-initialization. Bit0 represents the support when initialization. Set to 1 to enable."]
pub type PiRdlvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_GATE_EN` reader - Enables the PI gate training module. Bit1 represents the support when non-initialization. Bit0 represents the support when initialization. Set to 1 to enable."]
pub type PiRdlvlGateEnR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_GATE_EN` writer - Enables the PI gate training module. Bit1 represents the support when non-initialization. Bit0 represents the support when initialization. Set to 1 to enable."]
pub type PiRdlvlGateEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Indicates mask for the dfi_rdlvl_resp signal during data eye training."]
    #[inline(always)]
    pub fn pi_rdlvl_resp_mask(&self) -> PiRdlvlRespMaskR {
        PiRdlvlRespMaskR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Defines the DFI tRDLVL_EN timing parameter (in DFI clocks), the minimum cycles from a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion to the first read or MRR."]
    #[inline(always)]
    pub fn pi_tdfi_rdlvl_en(&self) -> PiTdfiRdlvlEnR {
        PiTdfiRdlvlEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Enables the PI data eye training module. Bit1 represents the support when non-initialization. Bit0 represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rdlvl_en(&self) -> PiRdlvlEnR {
        PiRdlvlEnR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Enables the PI gate training module. Bit1 represents the support when non-initialization. Bit0 represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_en(&self) -> PiRdlvlGateEnR {
        PiRdlvlGateEnR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indicates mask for the dfi_rdlvl_resp signal during data eye training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_resp_mask(&mut self) -> PiRdlvlRespMaskW<PiReg80Spec> {
        PiRdlvlRespMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Defines the DFI tRDLVL_EN timing parameter (in DFI clocks), the minimum cycles from a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion to the first read or MRR."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_rdlvl_en(&mut self) -> PiTdfiRdlvlEnW<PiReg80Spec> {
        PiTdfiRdlvlEnW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Enables the PI data eye training module. Bit1 represents the support when non-initialization. Bit0 represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_en(&mut self) -> PiRdlvlEnW<PiReg80Spec> {
        PiRdlvlEnW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Enables the PI gate training module. Bit1 represents the support when non-initialization. Bit0 represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_en(&mut self) -> PiRdlvlGateEnW<PiReg80Spec> {
        PiRdlvlGateEnW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 80\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_80::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_80::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg80Spec;
impl crate::RegisterSpec for PiReg80Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_80::R`](R) reader structure"]
impl crate::Readable for PiReg80Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_80::W`](W) writer structure"]
impl crate::Writable for PiReg80Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_80 to value 0"]
impl crate::Resettable for PiReg80Spec {
    const RESET_VALUE: u32 = 0;
}
