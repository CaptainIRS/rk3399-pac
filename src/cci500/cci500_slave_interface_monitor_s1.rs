#[doc = "Register `CCI500_SLAVE_INTERFACE_MONITOR_S1` reader"]
pub type R = crate::R<Cci500SlaveInterfaceMonitorS1Spec>;
#[doc = "Register `CCI500_SLAVE_INTERFACE_MONITOR_S1` writer"]
pub type W = crate::W<Cci500SlaveInterfaceMonitorS1Spec>;
#[doc = "Field `STALLED_AR_CHANNEL` reader - A transfer is stalled on the AR\n\nchannel.ARVALID is HIGH. ARREADY is\n\nLOW."]
pub type StalledArChannelR = crate::BitReader;
#[doc = "Field `STALLED_AR_CHANNEL` writer - A transfer is stalled on the AR\n\nchannel.ARVALID is HIGH. ARREADY is\n\nLOW."]
pub type StalledArChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLED_R_CHANNEL` reader - A transfer is stalled on the R channel.\n\nRVALID is HIGH.\n\nRREADY is LOW."]
pub type StalledRChannelR = crate::BitReader;
#[doc = "Field `STALLED_R_CHANNEL` writer - A transfer is stalled on the R channel.\n\nRVALID is HIGH.\n\nRREADY is LOW."]
pub type StalledRChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLED_AW_CHANNEL` reader - A transfer is stalled on the AW channel.\n\nAWVALID is HIGH.\n\nAWREADY is LOW."]
pub type StalledAwChannelR = crate::BitReader;
#[doc = "Field `STALLED_AW_CHANNEL` writer - A transfer is stalled on the AW channel.\n\nAWVALID is HIGH.\n\nAWREADY is LOW."]
pub type StalledAwChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLED_W_CHANNEL` reader - A transfer is stalled on the W channel.\n\nWVALID is HIGH.\n\nWREADY is LOW."]
pub type StalledWChannelR = crate::BitReader;
#[doc = "Field `STALLED_W_CHANNEL` writer - A transfer is stalled on the W channel.\n\nWVALID is HIGH.\n\nWREADY is LOW."]
pub type StalledWChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLED_B_CHANNEL` reader - A transfer is stalled on the B channel. BVALID\n\nis HIGH\n\nBREADY is LOW."]
pub type StalledBChannelR = crate::BitReader;
#[doc = "Field `STALLED_B_CHANNEL` writer - A transfer is stalled on the B channel. BVALID\n\nis HIGH\n\nBREADY is LOW."]
pub type StalledBChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLED_AC_CHANNEL` reader - A transfer is stalled on the AC channel.\n\nACVALID is HIGH.\n\nACREADY is LOW."]
pub type StalledAcChannelR = crate::BitReader;
#[doc = "Field `STALLED_AC_CHANNEL` writer - A transfer is stalled on the AC channel.\n\nACVALID is HIGH.\n\nACREADY is LOW."]
pub type StalledAcChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLED_CR_CHANNEL` reader - A transfer is stalled on the CR channel.\n\nCRVALID is HIGH.\n\nCRREADY is LOW."]
pub type StalledCrChannelR = crate::BitReader;
#[doc = "Field `STALLED_CR_CHANNEL` writer - A transfer is stalled on the CR channel.\n\nCRVALID is HIGH.\n\nCRREADY is LOW."]
pub type StalledCrChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLED_CD_CHANNEL` reader - A transfer is stalled on the CD channel.\n\nCDVALID is HIGH.\n\nCDREADY is LOW.\n\nACE slave only."]
pub type StalledCdChannelR = crate::BitReader;
#[doc = "Field `STALLED_CD_CHANNEL` writer - A transfer is stalled on the CD channel.\n\nCDVALID is HIGH.\n\nCDREADY is LOW.\n\nACE slave only."]
pub type StalledCdChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTSTANDING_READS` reader - Number of outstanding read transactions.\n\nFrom request\n\nhandshake to response or RACK for ACE\n\ninterfaces."]
pub type OutstandingReadsR = crate::FieldReader;
#[doc = "Field `OUTSTANDING_READS` writer - Number of outstanding read transactions.\n\nFrom request\n\nhandshake to response or RACK for ACE\n\ninterfaces."]
pub type OutstandingReadsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OUTSTANDING_WRITES` reader - Number of outstanding write transactions.\n\nFrom request handshake to response for ACE\n\nLite interfaces or WACK for ACE interfaces."]
pub type OutstandingWritesR = crate::FieldReader;
#[doc = "Field `OUTSTANDING_WRITES` writer - Number of outstanding write transactions.\n\nFrom request handshake to response for ACE\n\nLite interfaces or WACK for ACE interfaces."]
pub type OutstandingWritesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OUTSTANDING_SNOOPS` reader - Number of outstanding snoop requests or\n\nDVM messages. From request handshake to\n\nresponse or snoop data for a hit."]
pub type OutstandingSnoopsR = crate::FieldReader;
#[doc = "Field `OUTSTANDING_SNOOPS` writer - Number of outstanding snoop requests or\n\nDVM messages. From request handshake to\n\nresponse or snoop data for a hit."]
pub type OutstandingSnoopsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - A transfer is stalled on the AR\n\nchannel.ARVALID is HIGH. ARREADY is\n\nLOW."]
    #[inline(always)]
    pub fn stalled_ar_channel(&self) -> StalledArChannelR {
        StalledArChannelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A transfer is stalled on the R channel.\n\nRVALID is HIGH.\n\nRREADY is LOW."]
    #[inline(always)]
    pub fn stalled_r_channel(&self) -> StalledRChannelR {
        StalledRChannelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A transfer is stalled on the AW channel.\n\nAWVALID is HIGH.\n\nAWREADY is LOW."]
    #[inline(always)]
    pub fn stalled_aw_channel(&self) -> StalledAwChannelR {
        StalledAwChannelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A transfer is stalled on the W channel.\n\nWVALID is HIGH.\n\nWREADY is LOW."]
    #[inline(always)]
    pub fn stalled_w_channel(&self) -> StalledWChannelR {
        StalledWChannelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A transfer is stalled on the B channel. BVALID\n\nis HIGH\n\nBREADY is LOW."]
    #[inline(always)]
    pub fn stalled_b_channel(&self) -> StalledBChannelR {
        StalledBChannelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A transfer is stalled on the AC channel.\n\nACVALID is HIGH.\n\nACREADY is LOW."]
    #[inline(always)]
    pub fn stalled_ac_channel(&self) -> StalledAcChannelR {
        StalledAcChannelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A transfer is stalled on the CR channel.\n\nCRVALID is HIGH.\n\nCRREADY is LOW."]
    #[inline(always)]
    pub fn stalled_cr_channel(&self) -> StalledCrChannelR {
        StalledCrChannelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A transfer is stalled on the CD channel.\n\nCDVALID is HIGH.\n\nCDREADY is LOW.\n\nACE slave only."]
    #[inline(always)]
    pub fn stalled_cd_channel(&self) -> StalledCdChannelR {
        StalledCdChannelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Number of outstanding read transactions.\n\nFrom request\n\nhandshake to response or RACK for ACE\n\ninterfaces."]
    #[inline(always)]
    pub fn outstanding_reads(&self) -> OutstandingReadsR {
        OutstandingReadsR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of outstanding write transactions.\n\nFrom request handshake to response for ACE\n\nLite interfaces or WACK for ACE interfaces."]
    #[inline(always)]
    pub fn outstanding_writes(&self) -> OutstandingWritesR {
        OutstandingWritesR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Number of outstanding snoop requests or\n\nDVM messages. From request handshake to\n\nresponse or snoop data for a hit."]
    #[inline(always)]
    pub fn outstanding_snoops(&self) -> OutstandingSnoopsR {
        OutstandingSnoopsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - A transfer is stalled on the AR\n\nchannel.ARVALID is HIGH. ARREADY is\n\nLOW."]
    #[inline(always)]
    #[must_use]
    pub fn stalled_ar_channel(&mut self) -> StalledArChannelW<Cci500SlaveInterfaceMonitorS1Spec> {
        StalledArChannelW::new(self, 0)
    }
    #[doc = "Bit 1 - A transfer is stalled on the R channel.\n\nRVALID is HIGH.\n\nRREADY is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn stalled_r_channel(&mut self) -> StalledRChannelW<Cci500SlaveInterfaceMonitorS1Spec> {
        StalledRChannelW::new(self, 1)
    }
    #[doc = "Bit 2 - A transfer is stalled on the AW channel.\n\nAWVALID is HIGH.\n\nAWREADY is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn stalled_aw_channel(&mut self) -> StalledAwChannelW<Cci500SlaveInterfaceMonitorS1Spec> {
        StalledAwChannelW::new(self, 2)
    }
    #[doc = "Bit 3 - A transfer is stalled on the W channel.\n\nWVALID is HIGH.\n\nWREADY is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn stalled_w_channel(&mut self) -> StalledWChannelW<Cci500SlaveInterfaceMonitorS1Spec> {
        StalledWChannelW::new(self, 3)
    }
    #[doc = "Bit 4 - A transfer is stalled on the B channel. BVALID\n\nis HIGH\n\nBREADY is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn stalled_b_channel(&mut self) -> StalledBChannelW<Cci500SlaveInterfaceMonitorS1Spec> {
        StalledBChannelW::new(self, 4)
    }
    #[doc = "Bit 5 - A transfer is stalled on the AC channel.\n\nACVALID is HIGH.\n\nACREADY is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn stalled_ac_channel(&mut self) -> StalledAcChannelW<Cci500SlaveInterfaceMonitorS1Spec> {
        StalledAcChannelW::new(self, 5)
    }
    #[doc = "Bit 6 - A transfer is stalled on the CR channel.\n\nCRVALID is HIGH.\n\nCRREADY is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn stalled_cr_channel(&mut self) -> StalledCrChannelW<Cci500SlaveInterfaceMonitorS1Spec> {
        StalledCrChannelW::new(self, 6)
    }
    #[doc = "Bit 7 - A transfer is stalled on the CD channel.\n\nCDVALID is HIGH.\n\nCDREADY is LOW.\n\nACE slave only."]
    #[inline(always)]
    #[must_use]
    pub fn stalled_cd_channel(&mut self) -> StalledCdChannelW<Cci500SlaveInterfaceMonitorS1Spec> {
        StalledCdChannelW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Number of outstanding read transactions.\n\nFrom request\n\nhandshake to response or RACK for ACE\n\ninterfaces."]
    #[inline(always)]
    #[must_use]
    pub fn outstanding_reads(&mut self) -> OutstandingReadsW<Cci500SlaveInterfaceMonitorS1Spec> {
        OutstandingReadsW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Number of outstanding write transactions.\n\nFrom request handshake to response for ACE\n\nLite interfaces or WACK for ACE interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn outstanding_writes(&mut self) -> OutstandingWritesW<Cci500SlaveInterfaceMonitorS1Spec> {
        OutstandingWritesW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Number of outstanding snoop requests or\n\nDVM messages. From request handshake to\n\nresponse or snoop data for a hit."]
    #[inline(always)]
    #[must_use]
    pub fn outstanding_snoops(&mut self) -> OutstandingSnoopsW<Cci500SlaveInterfaceMonitorS1Spec> {
        OutstandingSnoopsW::new(self, 24)
    }
}
#[doc = "Slave Interface Monitor Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_slave_interface_monitor_s1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_slave_interface_monitor_s1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cci500SlaveInterfaceMonitorS1Spec;
impl crate::RegisterSpec for Cci500SlaveInterfaceMonitorS1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci500_slave_interface_monitor_s1::R`](R) reader structure"]
impl crate::Readable for Cci500SlaveInterfaceMonitorS1Spec {}
#[doc = "`write(|w| ..)` method takes [`cci500_slave_interface_monitor_s1::W`](W) writer structure"]
impl crate::Writable for Cci500SlaveInterfaceMonitorS1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCI500_SLAVE_INTERFACE_MONITOR_S1 to value 0"]
impl crate::Resettable for Cci500SlaveInterfaceMonitorS1Spec {
    const RESET_VALUE: u32 = 0;
}
