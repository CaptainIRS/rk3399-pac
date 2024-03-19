#[doc = "Register `DDR_PI_REG_42` reader"]
pub type R = crate::R<DdrPiReg42Spec>;
#[doc = "Register `DDR_PI_REG_42` writer"]
pub type W = crate::W<DdrPiReg42Spec>;
#[doc = "Field `PI_TDELAY_RDWR_2_BUS_IDLE_F0` reader - Indicates the delay from read or write to bus idle. Recommended\n\nsetting is delay time from read command that issued to the last\n\nread data received. The suffix \"_f0\" of the parameter name is\n\nomitted when in non-DFS mode."]
pub type PiTdelayRdwr2BusIdleF0R = crate::FieldReader;
#[doc = "Field `PI_TDELAY_RDWR_2_BUS_IDLE_F0` writer - Indicates the delay from read or write to bus idle. Recommended\n\nsetting is delay time from read command that issued to the last\n\nread data received. The suffix \"_f0\" of the parameter name is\n\nomitted when in non-DFS mode."]
pub type PiTdelayRdwr2BusIdleF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TDELAY_RDWR_2_BUS_IDLE_F1` reader - Indicates the delay from read or write to bus idle. Recommended\n\nsetting is delay time from read command that issued to the last\n\nread data received. The suffix \"_f1\" of the parameter name is\n\nomitted when in non-DFS mode."]
pub type PiTdelayRdwr2BusIdleF1R = crate::FieldReader;
#[doc = "Field `PI_TDELAY_RDWR_2_BUS_IDLE_F1` writer - Indicates the delay from read or write to bus idle. Recommended\n\nsetting is delay time from read command that issued to the last\n\nread data received. The suffix \"_f1\" of the parameter name is\n\nomitted when in non-DFS mode."]
pub type PiTdelayRdwr2BusIdleF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TDELAY_RDWR_2_BUS_IDLE_F2` reader - Indicates the delay from read or write to bus idle. Recommended\n\nsetting is delay time from read command that issued to the last\n\nread data received. The suffix \"_f2\" of the parameter name is\n\nomitted when in non-DFS mode."]
pub type PiTdelayRdwr2BusIdleF2R = crate::FieldReader;
#[doc = "Field `PI_TDELAY_RDWR_2_BUS_IDLE_F2` writer - Indicates the delay from read or write to bus idle. Recommended\n\nsetting is delay time from read command that issued to the last\n\nread data received. The suffix \"_f2\" of the parameter name is\n\nomitted when in non-DFS mode."]
pub type PiTdelayRdwr2BusIdleF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TMRR` reader - Indicates DRAM TMRR value in cycles."]
pub type PiTmrrR = crate::FieldReader;
#[doc = "Field `PI_TMRR` writer - Indicates DRAM TMRR value in cycles."]
pub type PiTmrrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Indicates the delay from read or write to bus idle. Recommended\n\nsetting is delay time from read command that issued to the last\n\nread data received. The suffix \"_f0\" of the parameter name is\n\nomitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdelay_rdwr_2_bus_idle_f0(&self) -> PiTdelayRdwr2BusIdleF0R {
        PiTdelayRdwr2BusIdleF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Indicates the delay from read or write to bus idle. Recommended\n\nsetting is delay time from read command that issued to the last\n\nread data received. The suffix \"_f1\" of the parameter name is\n\nomitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdelay_rdwr_2_bus_idle_f1(&self) -> PiTdelayRdwr2BusIdleF1R {
        PiTdelayRdwr2BusIdleF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Indicates the delay from read or write to bus idle. Recommended\n\nsetting is delay time from read command that issued to the last\n\nread data received. The suffix \"_f2\" of the parameter name is\n\nomitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdelay_rdwr_2_bus_idle_f2(&self) -> PiTdelayRdwr2BusIdleF2R {
        PiTdelayRdwr2BusIdleF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Indicates DRAM TMRR value in cycles."]
    #[inline(always)]
    pub fn pi_tmrr(&self) -> PiTmrrR {
        PiTmrrR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates the delay from read or write to bus idle. Recommended\n\nsetting is delay time from read command that issued to the last\n\nread data received. The suffix \"_f0\" of the parameter name is\n\nomitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdelay_rdwr_2_bus_idle_f0(&mut self) -> PiTdelayRdwr2BusIdleF0W<DdrPiReg42Spec> {
        PiTdelayRdwr2BusIdleF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Indicates the delay from read or write to bus idle. Recommended\n\nsetting is delay time from read command that issued to the last\n\nread data received. The suffix \"_f1\" of the parameter name is\n\nomitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdelay_rdwr_2_bus_idle_f1(&mut self) -> PiTdelayRdwr2BusIdleF1W<DdrPiReg42Spec> {
        PiTdelayRdwr2BusIdleF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Indicates the delay from read or write to bus idle. Recommended\n\nsetting is delay time from read command that issued to the last\n\nread data received. The suffix \"_f2\" of the parameter name is\n\nomitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdelay_rdwr_2_bus_idle_f2(&mut self) -> PiTdelayRdwr2BusIdleF2W<DdrPiReg42Spec> {
        PiTdelayRdwr2BusIdleF2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Indicates DRAM TMRR value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrr(&mut self) -> PiTmrrW<DdrPiReg42Spec> {
        PiTmrrW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_42::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_42::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg42Spec;
impl crate::RegisterSpec for DdrPiReg42Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_42::R`](R) reader structure"]
impl crate::Readable for DdrPiReg42Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_42::W`](W) writer structure"]
impl crate::Writable for DdrPiReg42Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_42 to value 0"]
impl crate::Resettable for DdrPiReg42Spec {
    const RESET_VALUE: u32 = 0;
}
