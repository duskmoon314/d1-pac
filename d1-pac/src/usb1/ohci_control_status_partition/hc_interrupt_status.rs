#[doc = "Register `HC_INTERRUPT_STATUS` reader"]
pub struct R(crate::R<HC_INTERRUPT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_INTERRUPT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_INTERRUPT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_INTERRUPT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HC_INTERRUPT_STATUS` writer"]
pub struct W(crate::W<HC_INTERRUPT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_INTERRUPT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HC_INTERRUPT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_INTERRUPT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROOT_HUB_STATUS_CHANGE` reader - RootHubStatusChange\n\nThis bit is set when the content of or the content of any of \\[ NumberofDownstreamPort \\]
has changed."]
pub type ROOT_HUB_STATUS_CHANGE_R = crate::BitReader<bool>;
#[doc = "Field `ROOT_HUB_STATUS_CHANGE` writer - RootHubStatusChange\n\nThis bit is set when the content of or the content of any of \\[ NumberofDownstreamPort \\]
has changed."]
pub type ROOT_HUB_STATUS_CHANGE_W<'a> =
    crate::BitWriter<'a, u32, HC_INTERRUPT_STATUS_SPEC, bool, 6>;
#[doc = "Field `FRAME_NUMBER_OVERFLOW` reader - FrameNumberOverflow\n\nThis bit is set when the MSb of (bit 15) changes value, from 0 to 1 or from 1 to 0, and after has been updated."]
pub type FRAME_NUMBER_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_NUMBER_OVERFLOW` writer - FrameNumberOverflow\n\nThis bit is set when the MSb of (bit 15) changes value, from 0 to 1 or from 1 to 0, and after has been updated."]
pub type FRAME_NUMBER_OVERFLOW_W<'a> = crate::BitWriter<'a, u32, HC_INTERRUPT_STATUS_SPEC, bool, 5>;
#[doc = "Field `UNRECOVERABLE_ERROR` reader - UnrecoverableError\n\nThis bit is set when HC detects a system error not related to USB. HC should not proceed with any processing nor signaling before the system error has been corrected. HCD clears this bit after HC has been reset."]
pub type UNRECOVERABLE_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `UNRECOVERABLE_ERROR` writer - UnrecoverableError\n\nThis bit is set when HC detects a system error not related to USB. HC should not proceed with any processing nor signaling before the system error has been corrected. HCD clears this bit after HC has been reset."]
pub type UNRECOVERABLE_ERROR_W<'a> = crate::BitWriter<'a, u32, HC_INTERRUPT_STATUS_SPEC, bool, 4>;
#[doc = "Field `RESUME_DETECTED` reader - ResumeDetected\n\nThis bit is set when HC detects that a device on the USB is asserting resume signaling. It is the transition from no resume signaling to resume signaling causing this bit to be set. This bit is not set when HCD sets the USBRseume state."]
pub type RESUME_DETECTED_R = crate::BitReader<bool>;
#[doc = "Field `RESUME_DETECTED` writer - ResumeDetected\n\nThis bit is set when HC detects that a device on the USB is asserting resume signaling. It is the transition from no resume signaling to resume signaling causing this bit to be set. This bit is not set when HCD sets the USBRseume state."]
pub type RESUME_DETECTED_W<'a> = crate::BitWriter<'a, u32, HC_INTERRUPT_STATUS_SPEC, bool, 3>;
#[doc = "Field `START_OF_FRAME` reader - StartofFrame\n\nThis bit is set by HC at each start of frame and after the update of HccaFrameNumber. HC also generates a SOF token at the same time."]
pub type START_OF_FRAME_R = crate::BitReader<bool>;
#[doc = "Field `START_OF_FRAME` writer - StartofFrame\n\nThis bit is set by HC at each start of frame and after the update of HccaFrameNumber. HC also generates a SOF token at the same time."]
pub type START_OF_FRAME_W<'a> = crate::BitWriter<'a, u32, HC_INTERRUPT_STATUS_SPEC, bool, 2>;
#[doc = "Field `WRITEBACK_DONE_HEAD` reader - WritebackDoneHead\n\nThis bit is set immediately after HC has written to. Further updates of the will not occur until this bit has been cleared. HCD should only clear this bit after it has saved the content of HccaDoneHead."]
pub type WRITEBACK_DONE_HEAD_R = crate::BitReader<bool>;
#[doc = "Field `WRITEBACK_DONE_HEAD` writer - WritebackDoneHead\n\nThis bit is set immediately after HC has written to. Further updates of the will not occur until this bit has been cleared. HCD should only clear this bit after it has saved the content of HccaDoneHead."]
pub type WRITEBACK_DONE_HEAD_W<'a> = crate::BitWriter<'a, u32, HC_INTERRUPT_STATUS_SPEC, bool, 1>;
#[doc = "Field `SCHEDULING_OVERRUN` reader - SchedulingOverrun\n\nThis bit is set when the USB schedule for the current Frame overruns and after the update of. A scheduling overrun will also cause the SchedulingOverrunCount of to be incremented."]
pub type SCHEDULING_OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `SCHEDULING_OVERRUN` writer - SchedulingOverrun\n\nThis bit is set when the USB schedule for the current Frame overruns and after the update of. A scheduling overrun will also cause the SchedulingOverrunCount of to be incremented."]
pub type SCHEDULING_OVERRUN_W<'a> = crate::BitWriter<'a, u32, HC_INTERRUPT_STATUS_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 6 - RootHubStatusChange\n\nThis bit is set when the content of or the content of any of \\[ NumberofDownstreamPort \\]
has changed."]
    #[inline(always)]
    pub fn root_hub_status_change(&self) -> ROOT_HUB_STATUS_CHANGE_R {
        ROOT_HUB_STATUS_CHANGE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - FrameNumberOverflow\n\nThis bit is set when the MSb of (bit 15) changes value, from 0 to 1 or from 1 to 0, and after has been updated."]
    #[inline(always)]
    pub fn frame_number_overflow(&self) -> FRAME_NUMBER_OVERFLOW_R {
        FRAME_NUMBER_OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - UnrecoverableError\n\nThis bit is set when HC detects a system error not related to USB. HC should not proceed with any processing nor signaling before the system error has been corrected. HCD clears this bit after HC has been reset."]
    #[inline(always)]
    pub fn unrecoverable_error(&self) -> UNRECOVERABLE_ERROR_R {
        UNRECOVERABLE_ERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - ResumeDetected\n\nThis bit is set when HC detects that a device on the USB is asserting resume signaling. It is the transition from no resume signaling to resume signaling causing this bit to be set. This bit is not set when HCD sets the USBRseume state."]
    #[inline(always)]
    pub fn resume_detected(&self) -> RESUME_DETECTED_R {
        RESUME_DETECTED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - StartofFrame\n\nThis bit is set by HC at each start of frame and after the update of HccaFrameNumber. HC also generates a SOF token at the same time."]
    #[inline(always)]
    pub fn start_of_frame(&self) -> START_OF_FRAME_R {
        START_OF_FRAME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - WritebackDoneHead\n\nThis bit is set immediately after HC has written to. Further updates of the will not occur until this bit has been cleared. HCD should only clear this bit after it has saved the content of HccaDoneHead."]
    #[inline(always)]
    pub fn writeback_done_head(&self) -> WRITEBACK_DONE_HEAD_R {
        WRITEBACK_DONE_HEAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - SchedulingOverrun\n\nThis bit is set when the USB schedule for the current Frame overruns and after the update of. A scheduling overrun will also cause the SchedulingOverrunCount of to be incremented."]
    #[inline(always)]
    pub fn scheduling_overrun(&self) -> SCHEDULING_OVERRUN_R {
        SCHEDULING_OVERRUN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - RootHubStatusChange\n\nThis bit is set when the content of or the content of any of \\[ NumberofDownstreamPort \\]
has changed."]
    #[inline(always)]
    pub fn root_hub_status_change(&mut self) -> ROOT_HUB_STATUS_CHANGE_W {
        ROOT_HUB_STATUS_CHANGE_W::new(self)
    }
    #[doc = "Bit 5 - FrameNumberOverflow\n\nThis bit is set when the MSb of (bit 15) changes value, from 0 to 1 or from 1 to 0, and after has been updated."]
    #[inline(always)]
    pub fn frame_number_overflow(&mut self) -> FRAME_NUMBER_OVERFLOW_W {
        FRAME_NUMBER_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 4 - UnrecoverableError\n\nThis bit is set when HC detects a system error not related to USB. HC should not proceed with any processing nor signaling before the system error has been corrected. HCD clears this bit after HC has been reset."]
    #[inline(always)]
    pub fn unrecoverable_error(&mut self) -> UNRECOVERABLE_ERROR_W {
        UNRECOVERABLE_ERROR_W::new(self)
    }
    #[doc = "Bit 3 - ResumeDetected\n\nThis bit is set when HC detects that a device on the USB is asserting resume signaling. It is the transition from no resume signaling to resume signaling causing this bit to be set. This bit is not set when HCD sets the USBRseume state."]
    #[inline(always)]
    pub fn resume_detected(&mut self) -> RESUME_DETECTED_W {
        RESUME_DETECTED_W::new(self)
    }
    #[doc = "Bit 2 - StartofFrame\n\nThis bit is set by HC at each start of frame and after the update of HccaFrameNumber. HC also generates a SOF token at the same time."]
    #[inline(always)]
    pub fn start_of_frame(&mut self) -> START_OF_FRAME_W {
        START_OF_FRAME_W::new(self)
    }
    #[doc = "Bit 1 - WritebackDoneHead\n\nThis bit is set immediately after HC has written to. Further updates of the will not occur until this bit has been cleared. HCD should only clear this bit after it has saved the content of HccaDoneHead."]
    #[inline(always)]
    pub fn writeback_done_head(&mut self) -> WRITEBACK_DONE_HEAD_W {
        WRITEBACK_DONE_HEAD_W::new(self)
    }
    #[doc = "Bit 0 - SchedulingOverrun\n\nThis bit is set when the USB schedule for the current Frame overruns and after the update of. A scheduling overrun will also cause the SchedulingOverrunCount of to be incremented."]
    #[inline(always)]
    pub fn scheduling_overrun(&mut self) -> SCHEDULING_OVERRUN_W {
        SCHEDULING_OVERRUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_interrupt_status](index.html) module"]
pub struct HC_INTERRUPT_STATUS_SPEC;
impl crate::RegisterSpec for HC_INTERRUPT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_interrupt_status::R](R) reader structure"]
impl crate::Readable for HC_INTERRUPT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_interrupt_status::W](W) writer structure"]
impl crate::Writable for HC_INTERRUPT_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HC_INTERRUPT_STATUS to value 0"]
impl crate::Resettable for HC_INTERRUPT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
