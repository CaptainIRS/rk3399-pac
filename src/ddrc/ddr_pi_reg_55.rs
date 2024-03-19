#[doc = "Register `DDR_PI_REG_55` reader"]
pub type R = crate::R<DdrPiReg55Spec>;
#[doc = "Register `DDR_PI_REG_55` writer"]
pub type W = crate::W<DdrPiReg55Spec>;
#[doc = "Field `PI_SWLVL_RD_SLICE_2` writer - Indicates software leveling read command in WDQ training for data\n\nslice 2."]
pub type PiSwlvlRdSlice2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SW_WDQLVL_RESP_2` reader - Indicates WDQ leveling response for data slice 2."]
pub type PiSwWdqlvlResp2R = crate::FieldReader;
#[doc = "Field `PI_SWLVL_WR_SLICE_3` writer - Indicates software leveling write command in WDQ training for data\n\nslice 3."]
pub type PiSwlvlWrSlice3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SWLVL_RD_SLICE_3` writer - Indicates software leveling read command in WDQ training for data\n\nslice 3."]
pub type PiSwlvlRdSlice3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:9 - Indicates WDQ leveling response for data slice 2."]
    #[inline(always)]
    pub fn pi_sw_wdqlvl_resp_2(&self) -> PiSwWdqlvlResp2R {
        PiSwWdqlvlResp2R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates software leveling read command in WDQ training for data\n\nslice 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_rd_slice_2(&mut self) -> PiSwlvlRdSlice2W<DdrPiReg55Spec> {
        PiSwlvlRdSlice2W::new(self, 0)
    }
    #[doc = "Bit 16 - Indicates software leveling write command in WDQ training for data\n\nslice 3."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_wr_slice_3(&mut self) -> PiSwlvlWrSlice3W<DdrPiReg55Spec> {
        PiSwlvlWrSlice3W::new(self, 16)
    }
    #[doc = "Bit 24 - Indicates software leveling read command in WDQ training for data\n\nslice 3."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_rd_slice_3(&mut self) -> PiSwlvlRdSlice3W<DdrPiReg55Spec> {
        PiSwlvlRdSlice3W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_55::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_55::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg55Spec;
impl crate::RegisterSpec for DdrPiReg55Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_55::R`](R) reader structure"]
impl crate::Readable for DdrPiReg55Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_55::W`](W) writer structure"]
impl crate::Writable for DdrPiReg55Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_55 to value 0"]
impl crate::Resettable for DdrPiReg55Spec {
    const RESET_VALUE: u32 = 0;
}
