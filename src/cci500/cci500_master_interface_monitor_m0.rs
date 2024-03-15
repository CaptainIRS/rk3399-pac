#[doc = "Register `CCI500_MASTER_INTERFACE_MONITOR_M0` reader"]
pub type R = crate::R<Cci500MasterInterfaceMonitorM0Spec>;
#[doc = "Register `CCI500_MASTER_INTERFACE_MONITOR_M0` writer"]
pub type W = crate::W<Cci500MasterInterfaceMonitorM0Spec>;
#[doc = "Field `STALLED_AR_CHANNEL` reader - A transfer is stalled on the AR channel.ARVALID is HIGH. ARREADY is LOW"]
pub type StalledArChannelR = crate::BitReader;
#[doc = "Field `STALLED_AR_CHANNEL` writer - A transfer is stalled on the AR channel.ARVALID is HIGH. ARREADY is LOW"]
pub type StalledArChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLED_R_CHANNEL` reader - A transfer is stalled on the R channel. RVALID is HIGH. RREADY is LOW."]
pub type StalledRChannelR = crate::BitReader;
#[doc = "Field `STALLED_R_CHANNEL` writer - A transfer is stalled on the R channel. RVALID is HIGH. RREADY is LOW."]
pub type StalledRChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLED_AW_CHANNEL` reader - A transfer is stalled on the AW channel. AWVALID is HIGH. AWREADY is LOW."]
pub type StalledAwChannelR = crate::BitReader;
#[doc = "Field `STALLED_AW_CHANNEL` writer - A transfer is stalled on the AW channel. AWVALID is HIGH. AWREADY is LOW."]
pub type StalledAwChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLED_W_CHANNEL` reader - A transfer is stalled on the W channel. WVALID is HIGH. WREADY is LOW."]
pub type StalledWChannelR = crate::BitReader;
#[doc = "Field `STALLED_W_CHANNEL` writer - A transfer is stalled on the W channel. WVALID is HIGH. WREADY is LOW."]
pub type StalledWChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLED_B_CHANNEL` reader - A transfer is stalled on the B channel. BVALID is HIGH BREADY is LOW."]
pub type StalledBChannelR = crate::BitReader;
#[doc = "Field `STALLED_B_CHANNEL` writer - A transfer is stalled on the B channel. BVALID is HIGH BREADY is LOW."]
pub type StalledBChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTSTANDING_READS` reader - Number of outstanding read transactions. From request handshake to response."]
pub type OutstandingReadsR = crate::FieldReader;
#[doc = "Field `OUTSTANDING_READS` writer - Number of outstanding read transactions. From request handshake to response."]
pub type OutstandingReadsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OUTSTANDING_WRITES` reader - Number of outstanding write transactions. From request handshake to response."]
pub type OutstandingWritesR = crate::FieldReader;
#[doc = "Field `OUTSTANDING_WRITES` writer - Number of outstanding write transactions. From request handshake to response."]
pub type OutstandingWritesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - A transfer is stalled on the AR channel.ARVALID is HIGH. ARREADY is LOW"]
    #[inline(always)]
    pub fn stalled_ar_channel(&self) -> StalledArChannelR {
        StalledArChannelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A transfer is stalled on the R channel. RVALID is HIGH. RREADY is LOW."]
    #[inline(always)]
    pub fn stalled_r_channel(&self) -> StalledRChannelR {
        StalledRChannelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A transfer is stalled on the AW channel. AWVALID is HIGH. AWREADY is LOW."]
    #[inline(always)]
    pub fn stalled_aw_channel(&self) -> StalledAwChannelR {
        StalledAwChannelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A transfer is stalled on the W channel. WVALID is HIGH. WREADY is LOW."]
    #[inline(always)]
    pub fn stalled_w_channel(&self) -> StalledWChannelR {
        StalledWChannelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A transfer is stalled on the B channel. BVALID is HIGH BREADY is LOW."]
    #[inline(always)]
    pub fn stalled_b_channel(&self) -> StalledBChannelR {
        StalledBChannelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Number of outstanding read transactions. From request handshake to response."]
    #[inline(always)]
    pub fn outstanding_reads(&self) -> OutstandingReadsR {
        OutstandingReadsR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of outstanding write transactions. From request handshake to response."]
    #[inline(always)]
    pub fn outstanding_writes(&self) -> OutstandingWritesR {
        OutstandingWritesR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - A transfer is stalled on the AR channel.ARVALID is HIGH. ARREADY is LOW"]
    #[inline(always)]
    #[must_use]
    pub fn stalled_ar_channel(&mut self) -> StalledArChannelW<Cci500MasterInterfaceMonitorM0Spec> {
        StalledArChannelW::new(self, 0)
    }
    #[doc = "Bit 1 - A transfer is stalled on the R channel. RVALID is HIGH. RREADY is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn stalled_r_channel(&mut self) -> StalledRChannelW<Cci500MasterInterfaceMonitorM0Spec> {
        StalledRChannelW::new(self, 1)
    }
    #[doc = "Bit 2 - A transfer is stalled on the AW channel. AWVALID is HIGH. AWREADY is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn stalled_aw_channel(&mut self) -> StalledAwChannelW<Cci500MasterInterfaceMonitorM0Spec> {
        StalledAwChannelW::new(self, 2)
    }
    #[doc = "Bit 3 - A transfer is stalled on the W channel. WVALID is HIGH. WREADY is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn stalled_w_channel(&mut self) -> StalledWChannelW<Cci500MasterInterfaceMonitorM0Spec> {
        StalledWChannelW::new(self, 3)
    }
    #[doc = "Bit 4 - A transfer is stalled on the B channel. BVALID is HIGH BREADY is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn stalled_b_channel(&mut self) -> StalledBChannelW<Cci500MasterInterfaceMonitorM0Spec> {
        StalledBChannelW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Number of outstanding read transactions. From request handshake to response."]
    #[inline(always)]
    #[must_use]
    pub fn outstanding_reads(&mut self) -> OutstandingReadsW<Cci500MasterInterfaceMonitorM0Spec> {
        OutstandingReadsW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Number of outstanding write transactions. From request handshake to response."]
    #[inline(always)]
    #[must_use]
    pub fn outstanding_writes(&mut self) -> OutstandingWritesW<Cci500MasterInterfaceMonitorM0Spec> {
        OutstandingWritesW::new(self, 16)
    }
}
#[doc = "Master Interface Monitor Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_master_interface_monitor_m0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_master_interface_monitor_m0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cci500MasterInterfaceMonitorM0Spec;
impl crate::RegisterSpec for Cci500MasterInterfaceMonitorM0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci500_master_interface_monitor_m0::R`](R) reader structure"]
impl crate::Readable for Cci500MasterInterfaceMonitorM0Spec {}
#[doc = "`write(|w| ..)` method takes [`cci500_master_interface_monitor_m0::W`](W) writer structure"]
impl crate::Writable for Cci500MasterInterfaceMonitorM0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCI500_MASTER_INTERFACE_MONITOR_M0 to value 0"]
impl crate::Resettable for Cci500MasterInterfaceMonitorM0Spec {
    const RESET_VALUE: u32 = 0;
}
