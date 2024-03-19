#[doc = "Register `DDR_PI_REG_78` reader"]
pub type R = crate::R<DdrPiReg78Spec>;
#[doc = "Register `DDR_PI_REG_78` writer"]
pub type W = crate::W<DdrPiReg78Spec>;
#[doc = "Field `PI_TDFI_RDLVL_RR` reader - Defines the DFI tRDLVL_RR timing parameter (in DFI clocks), the minimum cycles between read commands."]
pub type PiTdfiRdlvlRrR = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_RDLVL_RR` writer - Defines the DFI tRDLVL_RR timing parameter (in DFI clocks), the minimum cycles between read commands."]
pub type PiTdfiRdlvlRrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Defines the DFI tRDLVL_RR timing parameter (in DFI clocks), the minimum cycles between read commands."]
    #[inline(always)]
    pub fn pi_tdfi_rdlvl_rr(&self) -> PiTdfiRdlvlRrR {
        PiTdfiRdlvlRrR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Defines the DFI tRDLVL_RR timing parameter (in DFI clocks), the minimum cycles between read commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_rdlvl_rr(&mut self) -> PiTdfiRdlvlRrW<DdrPiReg78Spec> {
        PiTdfiRdlvlRrW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 78\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_78::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_78::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg78Spec;
impl crate::RegisterSpec for DdrPiReg78Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_78::R`](R) reader structure"]
impl crate::Readable for DdrPiReg78Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_78::W`](W) writer structure"]
impl crate::Writable for DdrPiReg78Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_78 to value 0"]
impl crate::Resettable for DdrPiReg78Spec {
    const RESET_VALUE: u32 = 0;
}
