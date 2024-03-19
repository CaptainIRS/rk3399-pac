#[doc = "Register `DDR_PI_REG_22` reader"]
pub type R = crate::R<DdrPiReg22Spec>;
#[doc = "Register `DDR_PI_REG_22` writer"]
pub type W = crate::W<DdrPiReg22Spec>;
#[doc = "Field `PI_CONTROL_ERROR_STATUS` reader - Identifies the source of any pi_control phyupd_req/ phylvl_req_cs_n errors. Value of 1 indicates a timing violation of the associated timing parameter Bit 8: pi_control triggered phyupd_resp_error. Bit 7: pi_control triggered phyupd_type3_error. Bit 6: pi_control triggered phyupd_type2_error. Bit 5: pi_control triggered phyupd_type1_error. Bit 4: pi_control triggered phyupd_type0_error. Bit 3: phylvl_resp_error. Bit 2: phylvl_max_error. Bit 1: phymstr_resp_error. Bit 0: phymstr_max_error."]
pub type PiControlErrorStatusR = crate::FieldReader<u16>;
#[doc = "Field `PI_CONTROL_ERROR_STATUS` writer - Identifies the source of any pi_control phyupd_req/ phylvl_req_cs_n errors. Value of 1 indicates a timing violation of the associated timing parameter Bit 8: pi_control triggered phyupd_resp_error. Bit 7: pi_control triggered phyupd_type3_error. Bit 6: pi_control triggered phyupd_type2_error. Bit 5: pi_control triggered phyupd_type1_error. Bit 4: pi_control triggered phyupd_type0_error. Bit 3: phylvl_resp_error. Bit 2: phylvl_max_error. Bit 1: phymstr_resp_error. Bit 0: phymstr_max_error."]
pub type PiControlErrorStatusW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PI_EXIT_AFTER_INIT_CALVL` reader - Releases the DFI bus after complete initialization CA training and requests DFI bus again for the remaining initialization training. The DFI bus release is for the controller to issue MRW/ZQ after the PI completes initialization CA leveling, based on the JEDEC protocol requirement."]
pub type PiExitAfterInitCalvlR = crate::BitReader;
#[doc = "Field `PI_EXIT_AFTER_INIT_CALVL` writer - Releases the DFI bus after complete initialization CA training and requests DFI bus again for the remaining initialization training. The DFI bus release is for the controller to issue MRW/ZQ after the PI completes initialization CA leveling, based on the JEDEC protocol requirement."]
pub type PiExitAfterInitCalvlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Identifies the source of any pi_control phyupd_req/ phylvl_req_cs_n errors. Value of 1 indicates a timing violation of the associated timing parameter Bit 8: pi_control triggered phyupd_resp_error. Bit 7: pi_control triggered phyupd_type3_error. Bit 6: pi_control triggered phyupd_type2_error. Bit 5: pi_control triggered phyupd_type1_error. Bit 4: pi_control triggered phyupd_type0_error. Bit 3: phylvl_resp_error. Bit 2: phylvl_max_error. Bit 1: phymstr_resp_error. Bit 0: phymstr_max_error."]
    #[inline(always)]
    pub fn pi_control_error_status(&self) -> PiControlErrorStatusR {
        PiControlErrorStatusR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Releases the DFI bus after complete initialization CA training and requests DFI bus again for the remaining initialization training. The DFI bus release is for the controller to issue MRW/ZQ after the PI completes initialization CA leveling, based on the JEDEC protocol requirement."]
    #[inline(always)]
    pub fn pi_exit_after_init_calvl(&self) -> PiExitAfterInitCalvlR {
        PiExitAfterInitCalvlR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Identifies the source of any pi_control phyupd_req/ phylvl_req_cs_n errors. Value of 1 indicates a timing violation of the associated timing parameter Bit 8: pi_control triggered phyupd_resp_error. Bit 7: pi_control triggered phyupd_type3_error. Bit 6: pi_control triggered phyupd_type2_error. Bit 5: pi_control triggered phyupd_type1_error. Bit 4: pi_control triggered phyupd_type0_error. Bit 3: phylvl_resp_error. Bit 2: phylvl_max_error. Bit 1: phymstr_resp_error. Bit 0: phymstr_max_error."]
    #[inline(always)]
    #[must_use]
    pub fn pi_control_error_status(&mut self) -> PiControlErrorStatusW<DdrPiReg22Spec> {
        PiControlErrorStatusW::new(self, 0)
    }
    #[doc = "Bit 16 - Releases the DFI bus after complete initialization CA training and requests DFI bus again for the remaining initialization training. The DFI bus release is for the controller to issue MRW/ZQ after the PI completes initialization CA leveling, based on the JEDEC protocol requirement."]
    #[inline(always)]
    #[must_use]
    pub fn pi_exit_after_init_calvl(&mut self) -> PiExitAfterInitCalvlW<DdrPiReg22Spec> {
        PiExitAfterInitCalvlW::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg22Spec;
impl crate::RegisterSpec for DdrPiReg22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_22::R`](R) reader structure"]
impl crate::Readable for DdrPiReg22Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_22::W`](W) writer structure"]
impl crate::Writable for DdrPiReg22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_22 to value 0x0001_0000"]
impl crate::Resettable for DdrPiReg22Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
