#[doc = "Register `DP_MODE_CTL` reader"]
pub type R = crate::R<DpModeCtlSpec>;
#[doc = "Register `DP_MODE_CTL` writer"]
pub type W = crate::W<DpModeCtlSpec>;
#[doc = "Field `FIELD4` reader - PHY DP Power State - power state for PHY DP lanes. Direct mapping \n\nto the PMA's A0 to A3 power states (A0 = 0b0001, A1 = 0b0010, A2 \n\n= 0b0100 and A3 = 0b1000). (Re-synchronized to PSM clock.) \n\nAutomatically, cleared to 0x0, upon \\[7:4\\]
== \\[3:0\\], i.e. upon the \n\ncompletion of the power state change. This eliminates the need to \n\nwrite this register back to 0x0. The initial power state is set to A2. \n\nNote: The PMA has 2 other defined power states, A4 and A5. These \n\nwill not be used by HPHY DP and are thus not provided in this \n\nregister. \n\nDrives \n\nxcvr_power_state_req_ln_&lt;>\\[3:0\\]
\n\n(after \n\nre-\n\nsynchronization to the PSM clock) for each enabled PHY DP lane."]
pub type Field4R = crate::FieldReader;
#[doc = "Field `FIELD4` writer - PHY DP Power State - power state for PHY DP lanes. Direct mapping \n\nto the PMA's A0 to A3 power states (A0 = 0b0001, A1 = 0b0010, A2 \n\n= 0b0100 and A3 = 0b1000). (Re-synchronized to PSM clock.) \n\nAutomatically, cleared to 0x0, upon \\[7:4\\]
== \\[3:0\\], i.e. upon the \n\ncompletion of the power state change. This eliminates the need to \n\nwrite this register back to 0x0. The initial power state is set to A2. \n\nNote: The PMA has 2 other defined power states, A4 and A5. These \n\nwill not be used by HPHY DP and are thus not provided in this \n\nregister. \n\nDrives \n\nxcvr_power_state_req_ln_&lt;>\\[3:0\\]
\n\n(after \n\nre-\n\nsynchronization to the PSM clock) for each enabled PHY DP lane."]
pub type Field4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIELD3` reader - PHY \n\nDP \n\nPower \n\nState \n\nAcknowledgement \n\n- \n\npower \n\nstate \n\nacknowledgement for PHY DP lanes. (Re-synchro- nized to APB \n\nclock.) \n\nAfter \n\nre-synchronization \n\nto APB clock, \n\nthis \n\nis \n\nthe AND of \n\nxcvr_power_state_ack\\[3:0\\]
for each enabled HPHY DP lane."]
pub type Field3R = crate::FieldReader;
#[doc = "Field `FIELD2` reader - DP link reset - Reset for DP link. 0 = reset asserted, 1 = reset de-\n\nasserted. Clearing this bit places all of the DP configured PMA lanes \n\ninto reset even when phy_reset_n is de-asserted. It is used to \n\nchange the DP configuration (i.e. number of active lanes) when a \n\nPHY reset is not possible (i.e. due to simultaneous USB operation)."]
pub type Field2R = crate::BitReader;
#[doc = "Field `FIELD2` writer - DP link reset - Reset for DP link. 0 = reset asserted, 1 = reset de-\n\nasserted. Clearing this bit places all of the DP configured PMA lanes \n\ninto reset even when phy_reset_n is de-asserted. It is used to \n\nchange the DP configuration (i.e. number of active lanes) when a \n\nPHY reset is not possible (i.e. due to simultaneous USB operation)."]
pub type Field2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD1` reader - Reserved"]
pub type Field1R = crate::FieldReader;
#[doc = "Field `FIELD0` reader - PHY DP lane disable - 0 = enable associated PHY DP lane; 1 = \n\ndisable/powerdown the associated PHY DP lane. This field is used \n\nto disable PHY lanes when not used. For example, for VESA DP Alt \n\nMode pin assignments A, C and E, this field would be used to \n\ndisable unused lanes for 1 or 2 lane DP configura- tions. \n\nAdditionally, any PMA lanes mapped to the PHY DP lane will be \n\ndisabled and powered down. \n\n• \\[12\\]
= PHY DP lane 0 \n\n• \\[13\\]
= PHY DP lane 1 \n\n• \\[14\\]
= PHY DP lane 2 \n\n• \\[15\\]
= PHY DP lane 3"]
pub type Field0R = crate::FieldReader;
#[doc = "Field `FIELD0` writer - PHY DP lane disable - 0 = enable associated PHY DP lane; 1 = \n\ndisable/powerdown the associated PHY DP lane. This field is used \n\nto disable PHY lanes when not used. For example, for VESA DP Alt \n\nMode pin assignments A, C and E, this field would be used to \n\ndisable unused lanes for 1 or 2 lane DP configura- tions. \n\nAdditionally, any PMA lanes mapped to the PHY DP lane will be \n\ndisabled and powered down. \n\n• \\[12\\]
= PHY DP lane 0 \n\n• \\[13\\]
= PHY DP lane 1 \n\n• \\[14\\]
= PHY DP lane 2 \n\n• \\[15\\]
= PHY DP lane 3"]
pub type Field0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - PHY DP Power State - power state for PHY DP lanes. Direct mapping \n\nto the PMA's A0 to A3 power states (A0 = 0b0001, A1 = 0b0010, A2 \n\n= 0b0100 and A3 = 0b1000). (Re-synchronized to PSM clock.) \n\nAutomatically, cleared to 0x0, upon \\[7:4\\]
== \\[3:0\\], i.e. upon the \n\ncompletion of the power state change. This eliminates the need to \n\nwrite this register back to 0x0. The initial power state is set to A2. \n\nNote: The PMA has 2 other defined power states, A4 and A5. These \n\nwill not be used by HPHY DP and are thus not provided in this \n\nregister. \n\nDrives \n\nxcvr_power_state_req_ln_&lt;>\\[3:0\\]
\n\n(after \n\nre-\n\nsynchronization to the PSM clock) for each enabled PHY DP lane."]
    #[inline(always)]
    pub fn field4(&self) -> Field4R {
        Field4R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PHY \n\nDP \n\nPower \n\nState \n\nAcknowledgement \n\n- \n\npower \n\nstate \n\nacknowledgement for PHY DP lanes. (Re-synchro- nized to APB \n\nclock.) \n\nAfter \n\nre-synchronization \n\nto APB clock, \n\nthis \n\nis \n\nthe AND of \n\nxcvr_power_state_ack\\[3:0\\]
for each enabled HPHY DP lane."]
    #[inline(always)]
    pub fn field3(&self) -> Field3R {
        Field3R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - DP link reset - Reset for DP link. 0 = reset asserted, 1 = reset de-\n\nasserted. Clearing this bit places all of the DP configured PMA lanes \n\ninto reset even when phy_reset_n is de-asserted. It is used to \n\nchange the DP configuration (i.e. number of active lanes) when a \n\nPHY reset is not possible (i.e. due to simultaneous USB operation)."]
    #[inline(always)]
    pub fn field2(&self) -> Field2R {
        Field2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Reserved"]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:15 - PHY DP lane disable - 0 = enable associated PHY DP lane; 1 = \n\ndisable/powerdown the associated PHY DP lane. This field is used \n\nto disable PHY lanes when not used. For example, for VESA DP Alt \n\nMode pin assignments A, C and E, this field would be used to \n\ndisable unused lanes for 1 or 2 lane DP configura- tions. \n\nAdditionally, any PMA lanes mapped to the PHY DP lane will be \n\ndisabled and powered down. \n\n• \\[12\\]
= PHY DP lane 0 \n\n• \\[13\\]
= PHY DP lane 1 \n\n• \\[14\\]
= PHY DP lane 2 \n\n• \\[15\\]
= PHY DP lane 3"]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PHY DP Power State - power state for PHY DP lanes. Direct mapping \n\nto the PMA's A0 to A3 power states (A0 = 0b0001, A1 = 0b0010, A2 \n\n= 0b0100 and A3 = 0b1000). (Re-synchronized to PSM clock.) \n\nAutomatically, cleared to 0x0, upon \\[7:4\\]
== \\[3:0\\], i.e. upon the \n\ncompletion of the power state change. This eliminates the need to \n\nwrite this register back to 0x0. The initial power state is set to A2. \n\nNote: The PMA has 2 other defined power states, A4 and A5. These \n\nwill not be used by HPHY DP and are thus not provided in this \n\nregister. \n\nDrives \n\nxcvr_power_state_req_ln_&lt;>\\[3:0\\]
\n\n(after \n\nre-\n\nsynchronization to the PSM clock) for each enabled PHY DP lane."]
    #[inline(always)]
    #[must_use]
    pub fn field4(&mut self) -> Field4W<DpModeCtlSpec> {
        Field4W::new(self, 0)
    }
    #[doc = "Bit 8 - DP link reset - Reset for DP link. 0 = reset asserted, 1 = reset de-\n\nasserted. Clearing this bit places all of the DP configured PMA lanes \n\ninto reset even when phy_reset_n is de-asserted. It is used to \n\nchange the DP configuration (i.e. number of active lanes) when a \n\nPHY reset is not possible (i.e. due to simultaneous USB operation)."]
    #[inline(always)]
    #[must_use]
    pub fn field2(&mut self) -> Field2W<DpModeCtlSpec> {
        Field2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - PHY DP lane disable - 0 = enable associated PHY DP lane; 1 = \n\ndisable/powerdown the associated PHY DP lane. This field is used \n\nto disable PHY lanes when not used. For example, for VESA DP Alt \n\nMode pin assignments A, C and E, this field would be used to \n\ndisable unused lanes for 1 or 2 lane DP configura- tions. \n\nAdditionally, any PMA lanes mapped to the PHY DP lane will be \n\ndisabled and powered down. \n\n• \\[12\\]
= PHY DP lane 0 \n\n• \\[13\\]
= PHY DP lane 1 \n\n• \\[14\\]
= PHY DP lane 2 \n\n• \\[15\\]
= PHY DP lane 3"]
    #[inline(always)]
    #[must_use]
    pub fn field0(&mut self) -> Field0W<DpModeCtlSpec> {
        Field0W::new(self, 12)
    }
}
#[doc = "DP Mode Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_mode_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_mode_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpModeCtlSpec;
impl crate::RegisterSpec for DpModeCtlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dp_mode_ctl::R`](R) reader structure"]
impl crate::Readable for DpModeCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_mode_ctl::W`](W) writer structure"]
impl crate::Writable for DpModeCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DP_MODE_CTL to value 0xc000"]
impl crate::Resettable for DpModeCtlSpec {
    const RESET_VALUE: u16 = 0xc000;
}
