#[doc = "Register `GRF_SOC_STATUS0` reader"]
pub type R = crate::R<GrfSocStatus0Spec>;
#[doc = "Register `GRF_SOC_STATUS0` writer"]
pub type W = crate::W<GrfSocStatus0Spec>;
#[doc = "Field `DDR0_CKE_STATUS` reader - status bit of ddr0_cke_status"]
pub type Ddr0CkeStatusR = crate::FieldReader;
#[doc = "Field `DDR0_CKE_STATUS` writer - status bit of ddr0_cke_status"]
pub type Ddr0CkeStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDR0_ZQ_STATUS_OUT` reader - status bit of ddr0_zq_status_out"]
pub type Ddr0ZqStatusOutR = crate::FieldReader;
#[doc = "Field `DDR0_ZQ_STATUS_OUT` writer - status bit of ddr0_zq_status_out"]
pub type Ddr0ZqStatusOutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDR0_PORT_BUSY` reader - status bit of ddr0_port_busy"]
pub type Ddr0PortBusyR = crate::BitReader;
#[doc = "Field `DDR0_PORT_BUSY` writer - status bit of ddr0_port_busy"]
pub type Ddr0PortBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR0_CONTROLLER_BUSY` reader - status bit of ddr0_controller_busy"]
pub type Ddr0ControllerBusyR = crate::BitReader;
#[doc = "Field `DDR0_CONTROLLER_BUSY` writer - status bit of ddr0_controller_busy"]
pub type Ddr0ControllerBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR0_REFRESH_IN_PROCESS` reader - status bit of ddr0_refresh_in_process"]
pub type Ddr0RefreshInProcessR = crate::BitReader;
#[doc = "Field `DDR0_REFRESH_IN_PROCESS` writer - status bit of ddr0_refresh_in_process"]
pub type Ddr0RefreshInProcessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR0_Q_ALMOST_FULL` reader - status bit of ddr0_q_almost_full"]
pub type Ddr0QAlmostFullR = crate::BitReader;
#[doc = "Field `DDR0_Q_ALMOST_FULL` writer - status bit of ddr0_q_almost_full"]
pub type Ddr0QAlmostFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR0_MEM_RST_VALID` reader - status bit of ddr0_mem_rst_valid"]
pub type Ddr0MemRstValidR = crate::BitReader;
#[doc = "Field `DDR0_MEM_RST_VALID` writer - status bit of ddr0_mem_rst_valid"]
pub type Ddr0MemRstValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR1_CKE_STATUS` reader - status bit of ddr1_cke_status"]
pub type Ddr1CkeStatusR = crate::FieldReader;
#[doc = "Field `DDR1_CKE_STATUS` writer - status bit of ddr1_cke_status"]
pub type Ddr1CkeStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDR1_ZQ_STATUS_OUT` reader - status bit of ddr1_zq_status_out"]
pub type Ddr1ZqStatusOutR = crate::FieldReader;
#[doc = "Field `DDR1_ZQ_STATUS_OUT` writer - status bit of ddr1_zq_status_out"]
pub type Ddr1ZqStatusOutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDR1_PORT_BUSY` reader - status bit of ddr1_port_busy"]
pub type Ddr1PortBusyR = crate::BitReader;
#[doc = "Field `DDR1_PORT_BUSY` writer - status bit of ddr1_port_busy"]
pub type Ddr1PortBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR1_CONTROLLER_BUSY` reader - status bit of ddr1_controller_busy"]
pub type Ddr1ControllerBusyR = crate::BitReader;
#[doc = "Field `DDR1_CONTROLLER_BUSY` writer - status bit of ddr1_controller_busy"]
pub type Ddr1ControllerBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR1_REFRESH_IN_PROCESS` reader - status bit of ddr1_refresh_in_process"]
pub type Ddr1RefreshInProcessR = crate::BitReader;
#[doc = "Field `DDR1_REFRESH_IN_PROCESS` writer - status bit of ddr1_refresh_in_process"]
pub type Ddr1RefreshInProcessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR1_Q_ALMOST_FULL` reader - status bit of ddr1_q_almost_full"]
pub type Ddr1QAlmostFullR = crate::BitReader;
#[doc = "Field `DDR1_Q_ALMOST_FULL` writer - status bit of ddr1_q_almost_full"]
pub type Ddr1QAlmostFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR1_MEM_RST_VALID` reader - status bit of ddr1_mem_rst_valid"]
pub type Ddr1MemRstValidR = crate::BitReader;
#[doc = "Field `DDR1_MEM_RST_VALID` writer - status bit of ddr1_mem_rst_valid"]
pub type Ddr1MemRstValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - status bit of ddr0_cke_status"]
    #[inline(always)]
    pub fn ddr0_cke_status(&self) -> Ddr0CkeStatusR {
        Ddr0CkeStatusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - status bit of ddr0_zq_status_out"]
    #[inline(always)]
    pub fn ddr0_zq_status_out(&self) -> Ddr0ZqStatusOutR {
        Ddr0ZqStatusOutR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - status bit of ddr0_port_busy"]
    #[inline(always)]
    pub fn ddr0_port_busy(&self) -> Ddr0PortBusyR {
        Ddr0PortBusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - status bit of ddr0_controller_busy"]
    #[inline(always)]
    pub fn ddr0_controller_busy(&self) -> Ddr0ControllerBusyR {
        Ddr0ControllerBusyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - status bit of ddr0_refresh_in_process"]
    #[inline(always)]
    pub fn ddr0_refresh_in_process(&self) -> Ddr0RefreshInProcessR {
        Ddr0RefreshInProcessR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - status bit of ddr0_q_almost_full"]
    #[inline(always)]
    pub fn ddr0_q_almost_full(&self) -> Ddr0QAlmostFullR {
        Ddr0QAlmostFullR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - status bit of ddr0_mem_rst_valid"]
    #[inline(always)]
    pub fn ddr0_mem_rst_valid(&self) -> Ddr0MemRstValidR {
        Ddr0MemRstValidR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - status bit of ddr1_cke_status"]
    #[inline(always)]
    pub fn ddr1_cke_status(&self) -> Ddr1CkeStatusR {
        Ddr1CkeStatusR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - status bit of ddr1_zq_status_out"]
    #[inline(always)]
    pub fn ddr1_zq_status_out(&self) -> Ddr1ZqStatusOutR {
        Ddr1ZqStatusOutR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - status bit of ddr1_port_busy"]
    #[inline(always)]
    pub fn ddr1_port_busy(&self) -> Ddr1PortBusyR {
        Ddr1PortBusyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - status bit of ddr1_controller_busy"]
    #[inline(always)]
    pub fn ddr1_controller_busy(&self) -> Ddr1ControllerBusyR {
        Ddr1ControllerBusyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - status bit of ddr1_refresh_in_process"]
    #[inline(always)]
    pub fn ddr1_refresh_in_process(&self) -> Ddr1RefreshInProcessR {
        Ddr1RefreshInProcessR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - status bit of ddr1_q_almost_full"]
    #[inline(always)]
    pub fn ddr1_q_almost_full(&self) -> Ddr1QAlmostFullR {
        Ddr1QAlmostFullR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - status bit of ddr1_mem_rst_valid"]
    #[inline(always)]
    pub fn ddr1_mem_rst_valid(&self) -> Ddr1MemRstValidR {
        Ddr1MemRstValidR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - status bit of ddr0_cke_status"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_cke_status(&mut self) -> Ddr0CkeStatusW<GrfSocStatus0Spec> {
        Ddr0CkeStatusW::new(self, 0)
    }
    #[doc = "Bits 2:3 - status bit of ddr0_zq_status_out"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_zq_status_out(&mut self) -> Ddr0ZqStatusOutW<GrfSocStatus0Spec> {
        Ddr0ZqStatusOutW::new(self, 2)
    }
    #[doc = "Bit 4 - status bit of ddr0_port_busy"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_port_busy(&mut self) -> Ddr0PortBusyW<GrfSocStatus0Spec> {
        Ddr0PortBusyW::new(self, 4)
    }
    #[doc = "Bit 5 - status bit of ddr0_controller_busy"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_controller_busy(&mut self) -> Ddr0ControllerBusyW<GrfSocStatus0Spec> {
        Ddr0ControllerBusyW::new(self, 5)
    }
    #[doc = "Bit 6 - status bit of ddr0_refresh_in_process"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_refresh_in_process(&mut self) -> Ddr0RefreshInProcessW<GrfSocStatus0Spec> {
        Ddr0RefreshInProcessW::new(self, 6)
    }
    #[doc = "Bit 7 - status bit of ddr0_q_almost_full"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_q_almost_full(&mut self) -> Ddr0QAlmostFullW<GrfSocStatus0Spec> {
        Ddr0QAlmostFullW::new(self, 7)
    }
    #[doc = "Bit 8 - status bit of ddr0_mem_rst_valid"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_mem_rst_valid(&mut self) -> Ddr0MemRstValidW<GrfSocStatus0Spec> {
        Ddr0MemRstValidW::new(self, 8)
    }
    #[doc = "Bits 16:17 - status bit of ddr1_cke_status"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_cke_status(&mut self) -> Ddr1CkeStatusW<GrfSocStatus0Spec> {
        Ddr1CkeStatusW::new(self, 16)
    }
    #[doc = "Bits 18:19 - status bit of ddr1_zq_status_out"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_zq_status_out(&mut self) -> Ddr1ZqStatusOutW<GrfSocStatus0Spec> {
        Ddr1ZqStatusOutW::new(self, 18)
    }
    #[doc = "Bit 20 - status bit of ddr1_port_busy"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_port_busy(&mut self) -> Ddr1PortBusyW<GrfSocStatus0Spec> {
        Ddr1PortBusyW::new(self, 20)
    }
    #[doc = "Bit 21 - status bit of ddr1_controller_busy"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_controller_busy(&mut self) -> Ddr1ControllerBusyW<GrfSocStatus0Spec> {
        Ddr1ControllerBusyW::new(self, 21)
    }
    #[doc = "Bit 22 - status bit of ddr1_refresh_in_process"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_refresh_in_process(&mut self) -> Ddr1RefreshInProcessW<GrfSocStatus0Spec> {
        Ddr1RefreshInProcessW::new(self, 22)
    }
    #[doc = "Bit 23 - status bit of ddr1_q_almost_full"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_q_almost_full(&mut self) -> Ddr1QAlmostFullW<GrfSocStatus0Spec> {
        Ddr1QAlmostFullW::new(self, 23)
    }
    #[doc = "Bit 24 - status bit of ddr1_mem_rst_valid"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_mem_rst_valid(&mut self) -> Ddr1MemRstValidW<GrfSocStatus0Spec> {
        Ddr1MemRstValidW::new(self, 24)
    }
}
#[doc = "SOC status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_status0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_status0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSocStatus0Spec;
impl crate::RegisterSpec for GrfSocStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_soc_status0::R`](R) reader structure"]
impl crate::Readable for GrfSocStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_soc_status0::W`](W) writer structure"]
impl crate::Writable for GrfSocStatus0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SOC_STATUS0 to value 0x03"]
impl crate::Resettable for GrfSocStatus0Spec {
    const RESET_VALUE: u32 = 0x03;
}
